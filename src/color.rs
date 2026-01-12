use crate::vec3;

pub type Color = vec3::Vec3;

// We don't implement fmt::Display here because that would conflict with the impl for Vec3.
pub fn color_to_string(color: &Color) -> String {
    let r: u8 = (255.999f64 * color.x) as u8;
    let g: u8 = (255.999f64 * color.y) as u8;
    let b: u8 = (255.999f64 * color.z) as u8;
    format!("{} {} {}\n", r, g, b)
}
