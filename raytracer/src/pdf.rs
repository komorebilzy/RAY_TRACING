use std::f64::consts::PI;

use crate::*;

pub trait Pdf {
    fn value(&self, direction: Vect3) -> f64;
    fn generate(&self) -> Vect3;
}

pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    // pub fn new(w: Vect3) -> Self {
    //     let mut uv = Onb::default();
    //     uv.build_from_w(w);
    //     Self { uvw: (uv) }
    // }
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
