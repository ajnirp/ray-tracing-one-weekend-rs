// See the README for how to build and run

use crate::color::{color_to_string, Color};
use crate::hit::Hit;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use std::rc::Rc;

mod color;
mod hit;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

// fn compute_ray_color(ray: &Ray) -> Color {
//     let sphere_center = Vec3::new(0f64, 0f64, -1f64);
//     let t = hit_sphere(&sphere_center, 0.5f64, &ray);
//     if t > 0f64 {
//         let normal = Vec3::unit_vec(&(ray.at(t) - sphere_center));
//         // Attenuate by 0.5 to prevent blowing out the whites
//         return Color::new(normal.x + 1f64, normal.y + 1f64, normal.z + 1f64) * 0.5f64;
//     }

//     let unit_direction = Vec3::unit_vec(&ray.dir);
//     let a = 0.5f64 * (unit_direction.y + 1f64);  // interpolation variable
//     Color::new(1f64, 1f64, 1f64)*(1f64-a) + Color::new(0.5f64, 0.7f64, 1f64)*a
// }

fn compute_ray_color(ray: &Ray, world: &HittableList) -> Color {
    match world.hit(&ray, 0f64, f64::MAX) {
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

// Returns the smallest value of the ray's parameteric variable t for which the ray
// intersects the sphere of radius `radius` centered at `center`.
fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - ray.orig;
    let a = ray.dir.len_sq();
    let h = ray.dir.dot(&oc);
    let c = oc.len_sq() - radius*radius;
    let discriminant = h*h - a*c;
    
    if discriminant < 0f64 { -1f64 }
    else { (h - discriminant.sqrt()) / a }
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

fn main() {
    // Image
    let aspect_ratio = 16f64 / 9f64;
    let image_width: u32 = 400;
    let image_height = compute_image_height(image_width, aspect_ratio);
    
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, 0f64, -1f64), 0.5f64)));
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, -100.5f64, -1f64), 100f64)));

    // Camera
    let focal_length = 1f64;
    let viewport_height = 2f64;
    let viewport_width = viewport_height * actual_aspect_ratio(image_width, image_height);
    let camera_center = Vec3::new(0f64, 0f64, 0f64);

    // Vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

    // Horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Location of the upper left pixel
    let viewport_upper_left = camera_center + Vec3::new(0f64, 0f64, -focal_length) - (viewport_u / 2f64) - (viewport_v / 2f64);
    let pixel_upper_left_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5f64);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for col in 0..image_height {
        let scanlines_remaining = image_height - col;
        if scanlines_remaining % 10u32 == 0 {
            eprintln!("Scanlines remaining: {}", image_height - col);
        }
        for row in 0..image_width {
            let pixel_center = pixel_upper_left_loc + (pixel_delta_u * row as f64) + (pixel_delta_v * col as f64);
            let ray = Ray::new(camera_center, pixel_center - camera_center);
            let pixel_color = compute_ray_color(&ray, &world);
            print!("{}", color_to_string(&pixel_color));
        }
    }
}
