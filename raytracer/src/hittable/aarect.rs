use std::f64::INFINITY;

use crate::*;
#[derive(Clone, Copy)]
pub struct XyRect<M: Material> {
    pub mp: M,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}
impl<M: Material> XyRect<M> {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: (mat),
            x0: (_x0),
            x1: (_x1),
            y0: (_y0),
            y1: (_y1),
            k: (_k),
        }
    }
}
impl<M: Material> Hittable for XyRect<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Vect3::new(self.x0, self.y0, self.k - 0.0001),
            Vect3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        let t = (self.k - _r.origin().z()) / _r.direction().z();
        if t < _t_min || t > _t_max {
            return None;
        }
        let x = _r.origin().x() + t * _r.direction().x();
        let y = _r.origin().y() + t * _r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vect3::new(0.0, 0.0, 1.0);
        let mut rec = HitRecord::new(t, _r.point_at_parameter(t), &self.mp);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.set_face_normal(_r, outward_normal);
        Some(rec)
    }
}

#[derive(Clone)]
pub struct XzRect<M: Material> {
    pub mp: M,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl<M: Material> XzRect<M> {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: (mat),
            x0: (_x0),
            x1: (_x1),
            z0: (_z0),
            z1: (_z1),
            k: (_k),
        }
    }
}
impl<M: Material> Hittable for XzRect<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Vect3::new(self.x0, self.k - 0.0001, self.z0),
            Vect3::new(self.x1, self.k + 0.0001, self.z1),
        );
        Some(output_box)
    }
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        let t = (self.k - _r.origin().y()) / _r.direction().y();
        if t < _t_min || t > _t_max {
            return None;
        }
        let x = _r.origin().x() + t * _r.direction().x();
        let z = _r.origin().z() + t * _r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let outward_normal = Vect3::new(0.0, 1.0, 0.0);
        let mut rec = HitRecord::new(t, _r.point_at_parameter(t), &self.mp);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.set_face_normal(_r, outward_normal);
        Some(rec)
    }
    fn pdf_value(&self, _o: Vect3, _v: Vect3) -> f64 {
        match self.hit(&Ray::new(_o, _v, 0.0), 0.0001, INFINITY) {
            Some(x) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squred = x.t * x.t * _v.squared_length();
                let cosine = (dot(_v, x.normal) / _v.length()).abs();
                distance_squred / (cosine * area)
            }
            None => 0.0,
        }
    }
    fn random(&self, _o: Vect3) -> Vect3 {
        let random_point = Vect3::new(
            random_double_rng(self.x0, self.x1),
            self.k,
            random_double_rng(self.z0, self.z1),
        );
        random_point - _o
    }
}

#[derive(Clone)]
pub struct YzRect<M: Material> {
    pub mp: M,
    pub z0: f64,
    pub z1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl<M: Material> YzRect<M> {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: (mat),
            z0: (_z0),
            z1: (_z1),
            y0: (_y0),
            y1: (_y1),
            k: (_k),
        }
    }
}
impl<M: Material> Hittable for YzRect<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Vect3::new(self.k - 0.0001, self.y0, self.z0),
            Vect3::new(self.k + 0.0001, self.y1, self.z1),
        );
        Some(output_box)
    }
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        let t = (self.k - _r.origin().x()) / _r.direction().x();
        if t < _t_min || t > _t_max {
            return None;
        }
        let z = _r.origin().z() + t * _r.direction().z();
        let y = _r.origin().y() + t * _r.direction().y();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vect3::new(1.0, 0.0, 0.0);
        let mut rec = HitRecord::new(t, _r.point_at_parameter(t), &self.mp);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.set_face_normal(_r, outward_normal);
        Some(rec)
    }
}
