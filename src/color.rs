use crate::interval::Interval;
use crate::vec3;

pub type Color = vec3::Vec3;

pub struct ColorBytes {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorBytes {
    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }
}

// We don't implement fmt::Display here because that would conflict with the impl for Vec3.
pub fn color_to_string(color: &Color) -> ColorBytes {
    let k_intensity = Interval::new(0.0, 0.999);

    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());
    
    ColorBytes {
        r: (256.0 * k_intensity.clamp(r)) as u8,
        g: (256.0 * k_intensity.clamp(g)) as u8,
        b: (256.0 * k_intensity.clamp(b)) as u8,
    }
}

// Transforms linear color to gamma space so that the human eye can more
// better see darker tones.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}