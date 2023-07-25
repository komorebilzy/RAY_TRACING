pub mod ray;
pub mod vect3;

pub use ray::*;
pub use vect3::*;

use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
pub fn random_double_rng(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
pub fn random_int(min: i64, max: i64) -> i64 {
    random_double_rng(min as f64, (max + 1) as f64) as i64
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        max
    } else {
        x
    }
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
