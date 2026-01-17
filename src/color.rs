use crate::interval::Interval;
use crate::vec3;

pub type Color = vec3::Vec3;

// We don't implement fmt::Display here because that would conflict with the impl for Vec3.
pub fn color_to_string(color: &Color) -> String {
    let k_intensity = Interval::new(0.0, 0.999);

    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());
    
    let r: u8 = (256.0 * k_intensity.clamp(r)) as u8;
    let g: u8 = (256.0 * k_intensity.clamp(g)) as u8;
    let b: u8 = (256.0 * k_intensity.clamp(b)) as u8;
    format!("{} {} {}\n", r, g, b)
}

// Transforms linear color to gamma space so that the human eye can more
// better see darker tones.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}