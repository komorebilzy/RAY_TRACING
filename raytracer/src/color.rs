use crate::*;
use image::RgbImage;
#[derive(Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn pos(y: u32, x: u32) -> Position {
        Position { x, y }
    }
}

pub fn write_color(img: &mut RgbImage, pos: Position, pixel_color: Vect3, samples_per_pixel: i32) {
    let mut r = pixel_color[0];
    let mut g = pixel_color[1];
    let mut b = pixel_color[2];
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    let pixel = img.get_pixel_mut(pos.x, pos.y);
    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    *pixel = image::Rgb([
        ((256_f64) * clamp(r, 0.0, 0.999)) as u8,
        ((256_f64) * clamp(g, 0.0, 0.999)) as u8,
        ((256_f64) * clamp(b, 0.0, 0.999)) as u8,
    ]);
}

// pub fn read_color(img: &RgbImage, pos: Position) -> Vect3 {
//     let pixel = img.get_pixel(pos.x, pos.y);
//     let r = (*pixel)[0];
//     let g = (*pixel)[1];
//     let b = (*pixel)[2];
//     Vect3::new(r as f64, g as f64, b as f64)
// }
