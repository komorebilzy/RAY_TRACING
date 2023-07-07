use crate::*;
#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vect3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(cen: Vect3, r: f64) -> Self {
        Self {
            center: (cen),
            radius: (r),
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vect3 = r.origin() - self.center;
        let a: f64 = dot(r.direction(), r.direction());
        let b: f64 = dot(oc, r.direction());
        let c: f64 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut temp: f64 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_max || temp < t_min {
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp > t_max || temp < t_min {
                return false;
            }
        }
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        // rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal: Vect3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }
}
