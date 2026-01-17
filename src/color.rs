use crate::interval::Interval;
use crate::vec3;

pub type Color = vec3::Vec3;

// We don't implement fmt::Display here because that would conflict with the impl for Vec3.
pub fn color_to_string(color: &Color) -> String {
    let k_intensity = Interval::new(0f64, 0.999f64);
    
    let r: u8 = (256f64 * k_intensity.clamp(color.x())) as u8;
    let g: u8 = (256f64 * k_intensity.clamp(color.y())) as u8;
    let b: u8 = (256f64 * k_intensity.clamp(color.z())) as u8;
    format!("{} {} {}\n", r, g, b)
}
