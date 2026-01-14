// A trait for objects that can be hit. I went with "Hit" instead of "Hittable"
// to follow the pattern of traits being named using verbs like "Debug, Clone, Copy, Add" etc.

use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Assumes outward normal is a unit vector. Updates the fields front_face and normal
    // based on the given ray and outward normal.
    pub fn new(point: &Vec3, ray: &Ray, t: f64, outward_normal: &Vec3) -> Self {
        let front_face = ray.dir.dot(outward_normal) < 0f64;
        Self {
            point: *point,
            t: t,
            front_face: front_face,
            normal: if front_face { *outward_normal } else { -*outward_normal }
        }
    }
}

pub trait Hit {
    // Returns Some(HitRecord) only if the ray hits the object, else None.
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
