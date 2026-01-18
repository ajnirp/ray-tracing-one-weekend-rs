use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterResult {
    scattered: Ray,  // The scattered ray
    attenuation: Color,  // How much the scattered ray should be attenuated
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
    // TODO: at some point, we should change the return type to Option<ScatterResult>
    // in order to deal with materials that scatter no light.
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let mut scatter_direction = *hit_record.normal() + Vec3::uniform_random_unit_vec();

        // Catch degenerate scatter directions. These result from uniformly sampled random unit vectors
        // that exactly cancel out the normal unit vector at the hit point.
        if scatter_direction.is_near_zero() {
            scatter_direction = *hit_record.normal();
        }

        ScatterResult {
            scattered: Ray::new(*hit_record.point(), scatter_direction),
            attenuation: self.albedo
        }
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let reflected = ray.dir().reflect(hit_record.normal());
        ScatterResult {
            scattered: Ray::new(*ray.orig(), reflected),
            attenuation: self.albedo,
        }
    }
}
