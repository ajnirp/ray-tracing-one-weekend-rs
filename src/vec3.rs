use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // Static ctor
    pub fn new(x: f64, y: f64, z: f64) -> Self { Vec3 { x: x, y: y, z: z, } }

    // Instance methods
    pub fn neg(&self) -> Self { Vec3 { x: -self.x, y: -self.y, z: -self.z } }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    // does not handle division by zero
    pub fn scale_inv(&mut self, scalar: f64) {
        self.scale(1f64 / scalar);
    }

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
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<&Self> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
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
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
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
