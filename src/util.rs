use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180f64
}

// Returns a random f64 in the range [0, 1)
pub fn random_f64() -> f64 {
    rand::random_range(0.0..1.0)
}
