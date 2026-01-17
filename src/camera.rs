use crate::color::{Color, color_to_string};
use crate::hit::Hit;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util::random_f64;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_upper_left_loc: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,  // guards against stack overflow from reflected rays
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
fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5f64, random_f64() - 0.5f64, 0f64)
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
        let image_height = compute_image_height(image_width, aspect_ratio);
        
        let focal_length = 1f64;
        let viewport_height = 2f64;
        let viewport_width = viewport_height * actual_aspect_ratio(image_width, image_height);
        let camera_center = Vec3::new(0f64, 0f64, 0f64);

        let center = Vec3::new(0f64, 0f64, 0f64);

        // Vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
        let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / (image_width as f64);  // horizontal / column
        let pixel_delta_v = viewport_v / (image_height as f64);  // vertical / row

        // Location of the upper left pixel
        let viewport_upper_left = camera_center + Vec3::new(0f64, 0f64, -focal_length) - (viewport_u / 2f64) - (viewport_v / 2f64);
        let pixel_upper_left_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5f64);

        Self {
            image_width: image_width,
            image_height: image_height,
            center: center,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel_upper_left_loc: pixel_upper_left_loc,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
        }
    }

    fn get_ray(&self, row: u32, col: u32) -> Ray {
        let offset = sample_square();
        let row_f64 = row as f64;
        let col_f64 = col as f64;
        let pixel_sample = self.pixel_upper_left_loc + (self.pixel_delta_u * (offset.x() + col_f64)) + (self.pixel_delta_v * (offset.y() + row_f64));
        Ray::new(self.center, pixel_sample - self.center)
    }

    // Computes the color produced by a ray hitting the world. If it doesn't, just
    // render the background.
    fn compute_ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(&ray, &Interval::new(0.0, f64::MAX)) {
            Some(hit_record) => {
                let direction = Vec3::uniform_random_unit_vec_on_hemisphere(hit_record.normal());
                return self.compute_ray_color(&Ray::new(*hit_record.point(), direction), depth+1, world) * 0.5;
            },
            None => {
                let unit_direction = Vec3::unit_vec(&ray.dir);
                let a = 0.5 * (unit_direction.y() + 1.0);  // interpolation variable
                Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
            }
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for row in 0..self.image_height {
            let scanlines_remaining = self.image_height - row;
            if scanlines_remaining % 10u32 == 0 {
                eprintln!("Scanlines remaining: {}", self.image_height - row);
            }
            for col in 0..self.image_width {
                let mut pixel_color = Color::new(0f64, 0f64, 0f64);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(row, col);
                    pixel_color += self.compute_ray_color(&ray, 0, world);
                }
                pixel_color /= self.samples_per_pixel as f64;
                print!("{}", color_to_string(&pixel_color));
            }
        }
    }
}
