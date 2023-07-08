use crate::*;
#[derive(Clone)]
pub struct Sphere {
    pub center: Vect3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(cen: Vect3, r: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center: (cen),
            radius: (r),
            mat_ptr: (m),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vect3 = r.origin() - self.center;
        let a: f64 = dot(r.direction(), r.direction());
        let b: f64 = dot(oc, r.direction());
        let c: f64 = dot(oc, oc) - self.radius * self.radius;
        // let rec: Option<HitRecord> = None;
        let discriminant: f64 = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut temp: f64 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_max || temp < t_min {
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp > t_max || temp < t_min {
                return None;
            }
        }

        let t = temp;
        let p = r.point_at_parameter(t);
        let mut rec = HitRecord::new(t, p, &self.mat_ptr);
        let outward_normal: Vect3 = (p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
