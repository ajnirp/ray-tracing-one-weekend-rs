use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.orig;
        let a = ray.dir.len_sq();
        let h = ray.dir.dot(&oc);
        let c = oc.len_sq() - self.radius*self.radius;
        
        let discriminant = h*h - a*c;
        if discriminant < 0f64 {
            return false;
        }

        let mut root = (h - discriminant.sqrt()) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + discriminant.sqrt()) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        hit_record.normal = (hit_record.point - self.center) / self.radius;

        true
    }
}
