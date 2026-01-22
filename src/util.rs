use rand::Rng;

fn str_to_f64(s: &str) -> Result<f64, &'static str> {
    s.parse::<f64>().map_err(|_| "Error parsing")
}

// Returns a tuple of (width, height).
// Example input: "16,9". Output: (16.0, 9.0).
pub fn parse_aspect_ratio(ratio_str: &str) -> Result<(f64, f64), &'static str> {
    let parts = ratio_str.split(",");
    let parts: Vec<&str> = parts.collect();
    if parts.len() != 2 {
        return Err("Expected a ratio string like 16,9.");
    }

    let width = str_to_f64(parts[0])?;
    let height = str_to_f64(parts[1])?;
    Ok((width, height))
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// Returns a random f64 in the range [0, 1)
pub fn random(min: f64, max: f64, rng: &mut rand::rngs::ThreadRng) -> f64 {
    rng.random_range(min..max)
}
