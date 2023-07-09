use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
pub fn random_double_rng(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
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
// pub static PI: f64 = 3.1415926535897932385;
// pub const PI: f64 = 3.141_592_653_589_793;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
