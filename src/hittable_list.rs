use crate::hit::Hit;
use crate::hit::HitRecord;
use crate::ray::Ray;

use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hit>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hit>) {
        self.objects.push(object);
    }
}

impl Hit for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_result: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;
        for object in self.objects.iter() {
            match object.hit(ray, ray_tmin, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    hit_result = Some(hit);
                },
                None => (),
            }
        }
        hit_result
    }
}
