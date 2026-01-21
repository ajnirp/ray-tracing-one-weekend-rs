use crate::color::{Color, color_to_string};
use crate::hit::Hit;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util::{degrees_to_radians, random};
use crate::vec3::Vec3;

use std::fs::File;
use std::io::{BufWriter, Write};

const MIN_T_TO_PREVENT_SHADOW_ACNE: f64 = 1e-3;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_upper_left_loc: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,  // guards against stack overflow from reflected rays

    defocus_angle_degrees: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

// Computes the image height and ensures that it's at least 1.
fn compute_image_height(image_width: u32, aspect_ratio: f64) -> u32 {
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    if image_height < 1 {
        1
    } else {
        image_height
    }
}

// Assumes that image_height is not 0. This is guaranteed if we use `compute_image_height`.
fn actual_aspect_ratio(image_width: u32, image_height: u32) -> f64 {
    (image_width as f64) / (image_height as f64)
}

// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
fn sample_square(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    Vec3::new(random(-0.5, 0.5, rng), random(-0.5, 0.5, rng), 0.0)
}

impl Camera {
    // view_up = the "up" vector as seen from the world frame
    // defocus_angle_degrees = variation angle of rays through each pixel, in degrees
    // focus_distance = distance from camera look_from to plane of perfect focus
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vertical_fov_degrees: f64, look_from: &Vec3, look_at: &Vec3, view_up: &Vec3, defocus_angle_degrees: f64, focus_distance: f64) -> Self {
        let image_height = compute_image_height(image_width, aspect_ratio);
        
        let camera_center  = *look_from;

        // Determine viewport dimensions
        let theta = degrees_to_radians(vertical_fov_degrees);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * actual_aspect_ratio(image_width, image_height);

        // Unit basis vectors for the camera coordinate frame
        let w = (*look_from - *look_at).unit_vec();  // vector going from `look_at` to `look_from`
        let u = view_up.cross(&w).unit_vec();
        let v = w.cross(&u);

        // Vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height *  -v;

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / (image_width as f64);  // horizontal / column
        let pixel_delta_v = viewport_v / (image_height as f64);  // vertical / row

        // Location of the upper left pixel
        let viewport_upper_left = camera_center - (focus_distance * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel_upper_left_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Camera defocus disk basis vectors
        let defocus_radius = focus_distance * (degrees_to_radians(defocus_angle_degrees / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel_upper_left_loc: pixel_upper_left_loc,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,

            defocus_angle_degrees: defocus_angle_degrees,
            defocus_disk_u: defocus_disk_u,
            defocus_disk_v: defocus_disk_v,
        }
    }

    fn sample_from_defocus_disk(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        let point = Vec3::uniform_random_in_unit_disk(rng);
        self.center + (point.x() * self.defocus_disk_u) + (point.y() * self.defocus_disk_v)
    }

    // Constructs a camera ray originating from a random point on the defocus disk and
    // directed at a randomly sampled point around the pixel location (col, row).
    fn get_ray(&self, row: u32, col: u32, rng: &mut rand::rngs::ThreadRng) -> Ray {
        let offset = sample_square(rng);
        let row = row as f64;
        let col = col as f64;
        let ray_origin = if self.defocus_angle_degrees <= 0.0 { self.center } else { self.sample_from_defocus_disk(rng) };
        let pixel_sample = self.pixel_upper_left_loc + ((col + offset.x()) * self.pixel_delta_u) + ((row + offset.y()) * (self.pixel_delta_v));
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    // Computes the color produced by a ray hitting the world. If it doesn't, just
    // render the background.
    fn compute_ray_color(&self, ray: &Ray, depth: u32, world: &HittableList, rng: &mut rand::rngs::ThreadRng) -> Color {
        if depth == self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(&ray, &Interval::new(MIN_T_TO_PREVENT_SHADOW_ACNE, f64::MAX)) {
            Some(hit_record) => {
                let scatter_result = hit_record.material().scatter(&ray, &hit_record, rng);
                match scatter_result {
                    Some(scatter_result) => {
                        let color_from_scattered_ray = self.compute_ray_color(scatter_result.scattered(), depth+1, world, rng);
                        *scatter_result.attenuation() * color_from_scattered_ray
                    },
                    None => Color::new(0.0, 0.0, 0.0),
                }
            },
            None => {
                let unit_direction = Vec3::unit_vec(ray.dir());
                let a = 0.5 * (unit_direction.y() + 1.0);  // interpolation variable
                (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn render(&self, world: &HittableList, file: &mut BufWriter<File>, rng: &mut rand::rngs::ThreadRng) -> std::io::Result<()> {
        write!(file, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        for row in 0..self.image_height {
            let scanlines_remaining = self.image_height - row;
            if scanlines_remaining % 10 == 0 {
                eprintln!("Scanlines remaining: {}", self.image_height - row);
            }
            for col in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(row, col, rng);
                    pixel_color += self.compute_ray_color(&ray, 0, world, rng);
                }
                pixel_color /= self.samples_per_pixel as f64;
                let color_bytes = color_to_string(&pixel_color);
                write!(file, "{} {} {}\n", color_bytes.r(), color_bytes.g(), color_bytes.b())?;
            }
        }

        Ok(())
    }
}
