use crate::*;

pub struct MovingSphere<M: Material> {
    pub center0: Vect3,
    pub center1: Vect3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(cen0: Vect3, cen1: Vect3, _time0: f64, _time1: f64, r: f64, m: M) -> Self {
        Self {
            center0: (cen0),
            center1: (cen1),
            time0: (_time0),
            time1: (_time1),
            radius: (r),
            mat_ptr: (m),
        }
    }

    pub fn center(&self, time: f64) -> Vect3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = (r.direction()).squared_length();
        let half_b = dot(&oc, &r.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let t = root;
        let p = r.point_at_parameter(t);
        let mut rec = HitRecord::new(t, p, &self.mat_ptr);
        let outward_normal: Vect3 = (p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            minimum: (self.center(time0) - Vect3::new(self.radius, self.radius, self.radius)),
            maximum: (self.center(time0) + Vect3::new(self.radius, self.radius, self.radius)),
        };
        let box1 = Aabb {
            minimum: (self.center(time1) - Vect3::new(self.radius, self.radius, self.radius)),
            maximum: (self.center(time1) + Vect3::new(self.radius, self.radius, self.radius)),
        };
        let output_box = surrounding_box(box0, box1);
        Some(output_box)
    }
}
