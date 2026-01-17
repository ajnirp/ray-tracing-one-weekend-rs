use std::fmt;
use std::ops;

use crate::util::random_f64;

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

    pub fn random() -> Self {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    // Generates a unit 3D vector from the uniform distribution. Uses rejection
    // sampling to generate a vector that lies on or within the unit sphere.
    // Then normalizes it. Why not just call random() and then normalize it?
    // Because that wouldn't be a uniform distribution. 
    fn uniform_random_unit_vec() -> Self {
        loop {
            let result = Vec3::random();
            // Also reject vectors very close to the origin to prevent rounding
            // towards zero and then dividing by zero.
            if result.len_sq() < 1e-60 || result.len_sq() > 1.0 {
                continue
            }
            return result / result.len();
        }
    }

    // Generates a unit 3D vector from the uniform distribution that lies on the
    // same hemisphere as the point of contact of the normal vector with the
    // sphere.
    pub fn uniform_random_unit_vec_on_hemisphere(normal: &Vec3) -> Self {
        let result = Vec3::uniform_random_unit_vec();
        if result.dot(normal) > 0.0 { result } else { -result }
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

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0f64 {
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
