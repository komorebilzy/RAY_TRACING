use std::ops;

use crate::*;

pub struct Onb {
    pub axis: [Vect3; 3],
}
impl ops::Index<usize> for Onb {
    type Output = Vect3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
impl ops::IndexMut<usize> for Onb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.axis[index]
    }
}
impl Onb {
    pub fn default() -> Self {
        Self {
            axis: [
                Vect3::new(0.0, 0.0, 0.0),
                Vect3::new(0.0, 0.0, 0.0),
                Vect3::new(0.0, 0.0, 0.0),
            ],
        }
    }
    pub fn u(&self) -> Vect3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vect3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vect3 {
        self.axis[2]
    }
    // pub fn local1(&self, a: f64, b: f64, c: f64) -> Vect3 {
    //     self.u() * a + self.v() * b + self.w() * c
    // }

    pub fn local2(&self, a: Vect3) -> Vect3 {
        self.u() * a.x() + self.v() * a.y() + self.w() * a.z()
    }
    pub fn build_from_w(&self, n: Vect3) -> Self {
        let a = if self.w().x().abs() > 0.9 {
            Vect3::new(0.0, 1.0, 0.0)
        } else {
            Vect3::new(1.0, 0.0, 0.0)
        };
        Self {
            axis: [
                cross(self.w(), self.v()),
                unit_vector(cross(self.w(), a)),
                unit_vector(n),
            ],
        }
    }
}
