// See the README for how to build and run

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use std::rc::Rc;

mod camera;
mod color;
mod hit;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    // World
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5f64, material_center)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));

    // Camera
    let aspect_ratio = 16f64 / 9f64;
    let image_width = 400u32;
    let samples_per_pixel = 10u32;
    let max_depth = 10u32;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(&world);
}
