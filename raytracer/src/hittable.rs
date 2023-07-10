use crate::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vect3,
    pub normal: Vect3,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}
impl HitRecord {
    pub fn new(tt: f64, pp: Vect3, m: &Rc<dyn Material>) -> Self {
        Self {
            t: (tt),
            p: (pp),
            normal: (Vect3::default()),
            front_face: (false),
            mat_ptr: (m.clone()),
            u: (0.0),
            v: (0.0),
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
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
