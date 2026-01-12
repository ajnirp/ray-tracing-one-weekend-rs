// A trait for objects that can be hit. I went with "Hit" instead of "Hittable"
// to follow the pattern of traits being named using verbs like "Debug, Clone, Copy, Add" etc.

use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hit {
    // Returns `true` only if the ray hits the object. If it does, updates the hit record.
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool;
}
