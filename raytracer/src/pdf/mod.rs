pub mod onb;
pub use onb::*;
use std::f64::consts::PI;

use crate::*;
pub trait Pdf {
    fn value(&self, direction: &Vect3) -> f64;
    fn generate(&self) -> Vect3;
}

// #[derive(Clone, Copy)]
// pub struct EmptyPdf {}
// impl Pdf for EmptyPdf {
//     fn generate(&self) -> Vect3 {
//         Vect3::default()
//     }
//     fn value(&self, _direction: Vect3) -> f64 {
//         0.0
//     }
// }
// pub const DEFAULT_PDF: EmptyPdf = EmptyPdf {};

#[derive(Clone)]
pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: &Vect3) -> Self {
        let mut uv = Onb::default();
        uv.build_from_w(w);
        Self { uvw: (uv) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vect3) -> f64 {
        let cosine = dot(&unit_vector(direction), &self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vect3 {
        self.uvw.local2(random_cosine_direction())
    }
}

#[derive(Clone)]
pub struct HittablePdf<'a, H: Hittable> {
    pub o: Vect3,
    pub ptr: &'a H,
}
impl<'a, H: Hittable> HittablePdf<'a, H> {
    pub fn new(p: &'a H, origin: &Vect3) -> Self {
        Self {
            o: (*origin),
            ptr: (p),
        }
    }
}

impl<'a, H: Hittable> Pdf for HittablePdf<'a, H> {
    fn value(&self, direction: &Vect3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vect3 {
        self.ptr.random(&self.o)
    }
}

#[derive(Clone)]
pub struct MixturePdf<'a, P0: Pdf, P1: Pdf> {
    pub p0: &'a P0,
    pub p1: &'a P1,
}
impl<'a, P0: Pdf, P1: Pdf> MixturePdf<'a, P0, P1> {
    pub fn new(p0: &'a P0, p1: &'a P1) -> Self {
        Self { p0, p1 }
    }
}
impl<'a, P0: Pdf, P1: Pdf> Pdf for MixturePdf<'a, P0, P1> {
    fn generate(&self) -> Vect3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
    fn value(&self, direction: &Vect3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vect3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vect3::new(x, y, z)
}
