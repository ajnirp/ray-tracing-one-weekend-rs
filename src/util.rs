use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Returns a random f64 in the range [0, 1)
pub fn random(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
