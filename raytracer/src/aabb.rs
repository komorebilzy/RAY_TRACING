use crate::*;
#[derive(Clone)]
pub struct Aabb {
    pub minimum: Vect3,
    pub maximum: Vect3,
}
impl Aabb {
    // pub fn default() -> Self {
    //     Self {
    //         minimum: (Vect3::default()),
    //         maximum: (Vect3::default()),
    //     }
    // }
    pub fn new(a: Vect3, b: Vect3) -> Self {
        Self {
            minimum: (a),
            maximum: (b),
        }
    }

    pub fn min(&self) -> Vect3 {
        self.minimum
    }
    pub fn max(&self) -> Vect3 {
        self.maximum
    }

    // pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
    //     let mut t_minn = t_min;
    //     let mut t_maxx = t_max;
    //     for a in 0..3 {
    //         let inv_d = 1.0_f64 / (r.direction()[a]);
    //         let mut t0 = (self.minimum[a] - r.origin()[a]) * inv_d;
    //         let mut t1 = (self.minimum[a] - r.origin()[a]) * inv_d;
    //         if inv_d < 0.0_f64 {
    //             std::mem::swap(&mut t0, &mut t1);
    //         }
    //         t_minn = t0.max(t_minn);
    //         t_maxx = t1.min(t_maxx);
    //         if t_max <= t_min {
    //             return false;
    //         }
    //     }
    //     true
    // }
}
pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Vect3::new(
        (box0.min().x()).min(box1.min().x()),
        (box0.min().y()).min(box1.min().y()),
        (box0.min().z()).min(box1.min().z()),
    );
    let big = Vect3::new(
        (box0.max().x()).max(box1.max().x()),
        (box0.max().y()).max(box1.max().y()),
        (box0.max().z()).max(box1.max().z()),
    );
    Aabb::new(small, big)
}
