// See the README for how to build and run

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::util::random;
use crate::vec3::Vec3;

use clap::Parser;

use std::fs::File;
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

// fn main() {
//     // World
//     let mut world = HittableList::new();
//     let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
//     let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
//     let material_left = Rc::new(Dielectric::new(1.5));
//     let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
//     let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
//     world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
//     world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
//     world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
//     world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
//     world.add(Rc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

//     // Camera
//     let aspect_ratio = 16.0 / 9.0;
//     let image_width = 400u32;
//     let samples_per_pixel = 100u32;
//     let max_depth = 50u32;
    
//     let vertical_fov_degrees = 20.0;
//     let look_from = Vec3::new(-2.0, 2.0, 1.0);
//     let look_at = Vec3::new(0.0, 0.0, -1.0);
//     let view_up = Vec3::new(0.0, 1.0, 0.0);
    
//     let defocus_angle_degrees = 10.0;
//     let focus_distance = (look_from - look_at).len();
    
//     let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vertical_fov_degrees, &look_from, &look_at, &view_up, defocus_angle_degrees, focus_distance);

//     // Render!
//     camera.render(&world);
// }

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 20)]
    samples_per_pixel: u32,

    #[arg(long, default_value_t = 50)]
    max_depth: u32,

    #[arg(long, default_value_t = String::from("img\\a.ppm"))]
    out_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let mut file = File::create(args.out_file)?;

    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new((a as f64) + 0.9 * random(0.0, 1.0), 0.2, (b as f64) + 0.9 * random(0.0, 1.0));
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let choose_material = random(0.0, 1.0);
                match choose_material {
                    0.0..0.8 => {
                        let albedo = Color::random_vec(0.0, 1.0) * Color::random_vec(0.0, 1.0);
                        let sphere_material = Rc::new(Lambertian::new(albedo));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    },
                    0.8..0.95 => {
                        let albedo = Color::random_vec(0.5, 1.0);
                        let fuzz = random(0.0, 0.5);
                        let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    },
                    _ => {
                        let sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));
    
    let aspect_ratio      = 16.0 / 9.0;
    let image_width       = 1200u32;
    let samples_per_pixel = args.samples_per_pixel;
    let max_depth         = args.max_depth;

    let vertical_fov_degrees = 20.0;
    let look_from = Vec3::new(13.0,2.0,3.0);
    let look_at  = Vec3::new(0.0,0.0,0.0);
    let view_up  = Vec3::new(0.0,1.0,0.0);

    let defocus_angle_degrees = 0.6;
    let focus_distance    = 10.0;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vertical_fov_degrees, &look_from, &look_at, &view_up, defocus_angle_degrees, focus_distance);

    camera.render(&world, &mut file)?;

    Ok(())
}
