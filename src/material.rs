use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterResult {
    scattered: Ray,  // The scattered ray
    attenuation: Color,  // Brightness of the scattered ray relative to the incoming ray
}

impl ScatterResult {
    pub fn scattered(&self) -> &Ray { &self.scattered }
    pub fn attenuation(&self) -> &Color { &self.attenuation }
}

// A trait for material types to implement.
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
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
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = *hit_record.normal() + Vec3::uniform_random_unit_vec();

        // Catch degenerate scatter directions. These result from uniformly sampled random unit vectors
        // that exactly cancel out the normal unit vector at the hit point.
        if scatter_direction.is_near_zero() {
            scatter_direction = *hit_record.normal();
        }

        Some(ScatterResult {
            scattered: Ray::new(*hit_record.point(), scatter_direction),
            attenuation: self.albedo
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,  // Used the randomize the direction of the reflected ray.
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray.dir().reflect(hit_record.normal());
        let fuzzed = reflected.unit_vec() + (self.fuzz * Vec3::uniform_random_unit_vec());
        let scattered = Ray::new(*hit_record.point(), fuzzed);
        if scattered.dir().dot(hit_record.normal()) > 0.0 {
            Some(ScatterResult {
                attenuation: self.albedo,
                scattered: scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self {
            // Disallow negative values.
            refractive_index: if refractive_index > 0.0 { refractive_index } else { 1.0 },
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let relative_refractive_index = if hit_record.front_face() { 1.0 / self.refractive_index } else { self.refractive_index };
        let unit_direction = ray.dir().unit_vec();
        let cos_theta = unit_direction.dot(hit_record.normal());
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = relative_refractive_index * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(hit_record.normal())
        } else {
            Vec3::refract(&unit_direction, hit_record.normal(), relative_refractive_index)
        };
        Some(ScatterResult {
            scattered: Ray::new(*hit_record.point(), direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
