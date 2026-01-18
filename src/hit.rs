// A trait for objects that can be hit. I went with "Hit" instead of "Hittable"
// to follow the pattern of traits being named using verbs like "Debug, Clone, Copy, Add" etc.

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}

impl HitRecord {
    // Assumes outward normal is a unit vector. Updates the fields front_face and normal
    // based on the given ray and outward normal.
    pub fn new(point: &Vec3, ray: &Ray, t: f64, outward_normal: &Vec3, material: Rc<dyn Material>) -> Self {
        let front_face = ray.dir().dot(outward_normal) < 0.0;
        Self {
            point: *point,
            t: t,
            front_face: front_face,
            normal: if front_face { *outward_normal } else { -*outward_normal },
            material: material,
        }
    }

    pub fn point(&self) -> &Vec3 { &self.point }
    pub fn normal(&self) -> &Vec3 { &self.normal }
    pub fn t(&self) -> f64 { self.t }
    pub fn front_face(&self) -> bool { self.front_face }
    pub fn material(&self) -> Rc<dyn Material> { self.material.clone() }
}

pub trait Hit {
    // Returns Some(HitRecord) only if the ray hits the object, else None.
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
