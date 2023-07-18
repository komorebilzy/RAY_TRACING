use std::f64::consts::PI;

use crate::*;

pub trait Pdf {
    fn value(&self, direction: Vect3) -> f64;
    fn generate(&self) -> Vect3;
}

#[derive(Clone, Copy)]
pub struct EmptyPdf {}
impl Pdf for EmptyPdf {
    fn generate(&self) -> Vect3 {
        Vect3::default()
    }
    fn value(&self, _direction: Vect3) -> f64 {
        0.0
    }
}
pub const DEFAULT_PDF: EmptyPdf = EmptyPdf {};

pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vect3) -> Self {
        let mut uv = Onb::default();
        uv.build_from_w(w);
        Self { uvw: (uv) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vect3) -> f64 {
        let cosine = dot(unit_vector(direction), self.uvw.w());
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

pub struct HittablePdf {
    pub o: Vect3,
    pub ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable>, origin: Vect3) -> Self {
        Self {
            o: (origin),
            ptr: (p),
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vect3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vect3 {
        self.ptr.random(self.o)
    }
}

pub struct MixturePdf {
    pub p: [Arc<dyn Pdf>; 2],
}
impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}
impl Pdf for MixturePdf {
    fn generate(&self) -> Vect3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
    fn value(&self, direction: Vect3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
}
