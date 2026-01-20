// See the README for how to build and run

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
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
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Rc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u32;
    let samples_per_pixel = 100u32;
    let max_depth = 50u32;
    let vertical_fov_degrees = 20.0;
    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vertical_fov_degrees, &look_from, &look_at, &view_up);

    // Render!
    camera.render(&world);
}
