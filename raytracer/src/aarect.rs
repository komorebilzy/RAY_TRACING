use crate::*;
pub struct XyRec {
    pub mp: Rc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}
impl XyRec {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Rc<dyn Material>) -> Self {
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
impl Hittable for XyRec {
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
