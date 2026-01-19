use std::fmt;
use std::ops;

use crate::util::random;

const NEAR_ZERO_TOLERANCE: f64 = 1e-8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Vec3 { x: x, y: y, z: z, } }

    // Negation, scaling self by a scalar, inverse scaling not yet implemented

    pub fn len_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn len(&self) -> f64 { self.len_sq().sqrt() }

    // Static methods to return new vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn unit_vec(&self) -> Self {
        *self / (self.len())
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    // Reflects a ray (whose direction is `self`) about a normal.
    // Callers are responsible for ensuring that `unit_normal` is a unit vector.
    pub fn reflect(&self, unit_normal: &Self) -> Self {
        *self - (2.0 * self.dot(unit_normal) * *unit_normal)
    }

    // Refracts the ray whose *normalized* direction vector is `ray`.
    // Callers are responsible for ensuring that `ray` and `normal` are *both* normalized.
    // Ray travels from the first medium to the second medium (e.g. air to glass).
    // `relative_refractive_index` = refractive index of first / refractive index of second.
    pub fn refract(ray: &Self, normal: &Self, relative_refractive_index: f64) -> Self {
        // The reference implementation uses 1.0 here, but if both are normalized this
        // shouldn't be needed. I left it out.
        let cos_theta = -ray.dot(normal);
        // Component of the refracted ray that is perpendicular to the normal.
        let out_perpendicular = relative_refractive_index * (*ray + cos_theta * *normal);
        // Component of the out ray that is parallel to the normal.
        let out_parallel = -(1.0 - out_perpendicular.len_sq()).abs().sqrt() * *normal;
        out_perpendicular + out_parallel
    }

    // Generates a unit 3D vector lying in the unit sphere. Uses rejection
    // sampling to ensure uniform sampling.
    pub fn uniform_random_unit_vec() -> Self {
        loop {
            let result = Vec3::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0));
            // Also reject vectors very close to the origin to prevent rounding
            // towards zero and then dividing by zero.
            if result.len_sq() > 1e-60 && result.len_sq() <= 1.0 {
                return result / result.len();
            }
        }
    }

    pub fn is_near_zero(&self) -> bool {
        self.x < NEAR_ZERO_TOLERANCE && self.y < NEAR_ZERO_TOLERANCE && self.z < NEAR_ZERO_TOLERANCE
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }        
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Self> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.sub(*rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Dividing vector by zero");
        }
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
