use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
// pub fn random_double1(min: f64, max: f64) -> f64 {
//     let mut rng = rand::thread_rng();
//     rng.gen_range(min..max)
// }

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
