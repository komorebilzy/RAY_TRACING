use crate::*;
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vect3,
    pub normal: Vect3,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: (0.0),
            p: (Vect3::default()),
            normal: (Vect3::default()),
            front_face: (false),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vect3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
}
