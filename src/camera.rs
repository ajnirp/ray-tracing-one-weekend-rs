use crate::color::{Color, color_to_string};
use crate::hit::Hit;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_upper_left_loc: Vec3,
}

fn compute_ray_color(ray: &Ray, world: &HittableList) -> Color {
    match world.hit(&ray, &Interval::new(0f64, f64::MAX)) {
        Some(hit_record) => {
            (hit_record.normal + Color::new(1f64, 1f64, 1f64)) * 0.5f64
        },
        None => {
            let unit_direction = Vec3::unit_vec(&ray.dir);
            let a = 0.5f64 * (unit_direction.y + 1f64);  // interpolation variable
            Color::new(1f64, 1f64, 1f64)*(1f64-a) + Color::new(0.5f64, 0.7f64, 1f64)*a
        }
    }
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

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
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
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Location of the upper left pixel
        let viewport_upper_left = camera_center + Vec3::new(0f64, 0f64, -focal_length) - (viewport_u / 2f64) - (viewport_v / 2f64);
        let pixel_upper_left_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5f64);

        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: center,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel_upper_left_loc: pixel_upper_left_loc,
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for col in 0..self.image_height {
            let scanlines_remaining = self.image_height - col;
            if scanlines_remaining % 10u32 == 0 {
                eprintln!("Scanlines remaining: {}", self.image_height - col);
            }
            for row in 0..self.image_width {
                let pixel_center = self.pixel_upper_left_loc + (self.pixel_delta_u * row as f64) + (self.pixel_delta_v * col as f64);
                let ray = Ray::new(self.center, pixel_center - self.center);
                let pixel_color = compute_ray_color(&ray, &world);
                print!("{}", color_to_string(&pixel_color));
            }
        }
    }
}
