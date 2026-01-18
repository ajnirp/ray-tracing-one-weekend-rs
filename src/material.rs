use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterResult {
    // The scattered ray.
    scattered: Ray,

    // How much the scattered ray should be attenuated.
    attenuation: Color,
}

impl ScatterResult {
    pub fn scattered(&self) -> &Ray { &self.scattered }
    pub fn attenuation(&self) -> &Color { &self.attenuation }
}

// A trait for material types to implement.
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    // In this implementation we always scatter an incoming ray, never absorb.
    // `albedo` denotes the attenuation experienced by a scattered ray. When
    // it is 1.0, the scattered ray has the same brightness as the incoming ray
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: albedo,
        }
    }

    pub fn albedo(&self) -> &Color { &self.albedo }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let scatter_direction = *hit_record.normal() + Vec3::uniform_random_unit_vec();
        let scattered_ray = Ray::new(*hit_record.point(), scatter_direction);
        ScatterResult { scattered: scattered_ray, attenuation: *self.albedo() }
    }
}
