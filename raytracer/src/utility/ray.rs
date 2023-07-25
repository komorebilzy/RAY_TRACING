use crate::vect3::*;
#[derive(Clone, Copy)]
pub struct Ray {
    pub a: Vect3,
    pub b: Vect3,
    pub tm: f64,
}
impl Ray {
    // pub fn default() -> Self {
    //     Self {
    //         a: (Vect3::default()),
    //         b: (Vect3::default()),
    //         tm: 0.0,
    //     }
    // }
    pub fn new(a: Vect3, b: Vect3, t: f64) -> Self {
        Self {
            a: (a),
            b: (b),
            tm: (t),
        }
    }
    pub fn origin(&self) -> Vect3 {
        self.a
    }
    pub fn direction(&self) -> Vect3 {
        self.b
    }
    pub fn point_at_parameter(self, t: f64) -> Vect3 {
        self.a + self.b * t
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
}
