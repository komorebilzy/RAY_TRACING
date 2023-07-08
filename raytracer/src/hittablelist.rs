use crate::*;
use std::option::Option;
use std::rc::Rc;
pub struct HitableList {
    objects: Vec<Rc<dyn Hittable>>,
}
impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}
impl Hittable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;
        for i in &self.objects {
            let temp_rec = i.hit(r, t_min, closest_so_far);
            match temp_rec {
                Some(x) => {
                    rec = Some(x.clone());
                    closest_so_far = x.t;
                }
                None => {}
            }
        }
        rec
    }
}

// impl Hittable for HitableList {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
//         let mut hit_anything: bool = false;
//         let mut closest_so_far: f64 = t_max;
//         let mut temp_rec = HitRecord::new();
//         for i in self.objects.clone() {
//             if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
//                 hit_anything = true;
//                 closest_so_far = temp_rec.t;
//                 *rec = temp_rec;
//             }
//         }
//         hit_anything
//     }
// }
