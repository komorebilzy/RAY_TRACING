use crate::*;
use std::option::Option;
pub struct HitableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
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
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = Aabb::new(Vect3::default(), Vect3::default());
        let mut first_box = true;
        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                None => {
                    return None;
                }
                Some(x) => {
                    output_box = if first_box {
                        x
                    } else {
                        surrounding_box(output_box, x)
                    };
                    first_box = false;
                }
            }
        }
        Some(output_box)
    }
    fn pdf_value(&self, o: Vect3, v: Vect3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }
    fn random(&self, o: Vect3) -> Vect3 {
        let int_size: i64 = self.objects.len() as i64;
        self.objects[random_int(0, int_size - 1) as usize].random(o)
    }
}
