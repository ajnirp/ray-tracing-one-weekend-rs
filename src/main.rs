// See the README for how to build and run

use crate::hittable_list::HittableList;
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use std::rc::Rc;

mod camera;
mod color;
mod hit;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, 0f64, -1f64), 0.5f64)));
    world.add(Rc::new(Sphere::new(Vec3::new(0f64, -100.5f64, -1f64), 100f64)));

    // Camera
    let aspect_ratio = 16f64 / 9f64;
    let image_width = 400u32;
    let samples_per_pixel = 10u32;
    let max_depth = 10u32;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(&world);
}
