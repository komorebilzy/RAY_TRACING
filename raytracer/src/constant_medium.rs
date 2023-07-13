use crate::*;
pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f64,
}
impl ConstantMedium {
    // pub fn new1(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: (b),
    //         phase_function: (Rc::new(Isotropic::new2(a))),
    //         neg_inv_density: (-1.0 / d),
    //     }
    // }
    pub fn new2(b: Rc<dyn Hittable>, d: f64, c: Vect3) -> Self {
        Self {
            boundary: (b),
            phase_function: (Rc::new(Isotropic::new1(c))),
            neg_inv_density: (-1.0 / d),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_double() < 0.00001;
        match self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            Some(x) => match self.boundary.hit(r, x.t + 0.0001, f64::INFINITY) {
                Some(y) => {
                    if debugging {
                        eprintln!("This is an error message");
                    }
                    let mut rec1 = x;
                    let mut rec2 = y;
                    if rec1.t < t_min {
                        rec1.t = t_min;
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max;
                    }
                    if rec1.t >= rec2.t {
                        return None;
                    }
                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }
                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * random_double().ln();
                    if hit_distance > distance_inside_boundary {
                        return None;
                    }
                    let t = rec1.t + hit_distance / ray_length;
                    let mut ans = HitRecord::new(t, r.point_at_parameter(t), &self.phase_function);
                    ans.normal = Vect3::new(1.0, 0.0, 0.0);
                    ans.front_face = true;
                    Some(ans)
                }
                None => None,
            },
            None => None,
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
