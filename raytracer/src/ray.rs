use crate::*;
#[derive(Clone, Copy)]
pub struct Ray {
    pub a: Vect3,
    pub b: Vect3,
}
impl Ray {
    pub fn new(a: Vect3, b: Vect3) -> Self {
        Self { a: (a), b: (b) }
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
}
