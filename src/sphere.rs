use crate::hit::{Hit, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: center,
            radius: if radius < 0.0 { 0.0 } else { radius },
            material: material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - *ray.orig();
        let a = ray.dir().len_sq();
        let h = ray.dir().dot(&oc);
        let c = oc.len_sq() - self.radius*self.radius;
        
        let discriminant = h*h - a*c;
        if discriminant < 0f64 {
            return None;
        }

        let mut root = (h - discriminant.sqrt()) / a;
        if root <= ray_t.min() || root >= ray_t.max() {
            root = (h + discriminant.sqrt()) / a;
            if root <= ray_t.min() || root >= ray_t.max() {
                return None;
            }
        }

        let point =  ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(&point, ray, root, &outward_normal, self.material.clone()))
    }
}
