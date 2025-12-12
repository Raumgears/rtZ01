use std::io::Write;
use crate::{basics::Vec3, utils::clamp};

// Color is just 3 fields, as a Vec3
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32, gamma: f64) {
    if (pixel_color.x() == 0.0) && (pixel_color.y() == 0.0) && (pixel_color.z() == 0.0) {
        writeln!(out, "0 0 0").expect("writing color");
        return;
    }

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).powf(1.0/gamma);
    g = (scale * g).powf(1.0/gamma);
    b = (scale * b).powf(1.0/gamma);

    // Write the translated [0, 255] value of each color component
    writeln!(
        out,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}