// use image::flat::SampleLayout;

use crate::*;
// #[derive(Default)]
pub struct BvhNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub boxx: Aabb,
}

pub fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: i64) -> std::cmp::Ordering {
    match a.bounding_box(0.0, 0.0) {
        None => std::cmp::Ordering::Less,
        Some(x) => match b.bounding_box(0.0, 0.0) {
            None => {
                eprintln!("No bounding box in bvh_node constructor");
                std::cmp::Ordering::Less
            }
            Some(y) => {
                if x.min().e[axis as usize] < y.min().e[axis as usize] {
                    std::cmp::Ordering::Less
                } else if x.min().e[axis as usize] > y.min().e[axis as usize] {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        },
    }
}

pub fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn prinew(
        src_objects: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut ans = BvhNode {
            left: (None),
            right: (None),
            boxx: (Aabb::default()),
        };

        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = end - start;
        if object_span == 1 {
            ans.left = Some(src_objects.remove(start));
            ans.right = None;
        } else if object_span == 2 {
            if comparator(src_objects[start].as_ref(), src_objects[start + 1].as_ref())
                == std::cmp::Ordering::Less
            {
                ans.right = Some(src_objects.remove(start + 1));
                ans.left = Some(src_objects.remove(start));
            } else {
                ans.left = Some(src_objects.remove(start + 1));
                ans.right = Some(src_objects.remove(start));
            }
        } else {
            src_objects[start..end].sort_by(|a, b| comparator(a.as_ref(), b.as_ref()));
            // src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            ans.right = Some(Box::new(BvhNode::prinew(
                src_objects,
                mid,
                end,
                time0,
                time1,
            )));
            ans.left = Some(Box::new(BvhNode::prinew(
                src_objects,
                start,
                mid,
                time0,
                time1,
            )));
        }

        let left = if ans.left.is_some() {
            ans.left.as_ref().unwrap().bounding_box(time0, time1)
        } else {
            None
        };

        let right = if ans.right.is_some() {
            ans.right.as_ref().unwrap().bounding_box(time0, time1)
        } else {
            None
        };

        match left {
            Some(x) => match right {
                Some(y) => {
                    ans.boxx = surrounding_box(x, y);
                }
                None => {
                    ans.boxx = Aabb::new(Vect3::new(0.0, 0.0, 0.0), Vect3::new(0.0, 0.0, 0.0));
                }
            },
            None => {
                ans.boxx = Aabb::new(Vect3::new(0.0, 0.0, 0.0), Vect3::new(0.0, 0.0, 0.0));
            }
        }
        ans
    }

    pub fn new(mut list: HitableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        BvhNode::prinew(&mut list.objects, 0_usize, len, time0, time1)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.boxx.clone())
    }
    // fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    //     if !self.boxx.hit(r, t_min, t_max) {
    //         return None;
    //     }
    //     let hit_left = self.left.as_ref().unwrap().hit(r, t_min, t_max);
    //     match hit_left {
    //         None => self.right.as_ref().unwrap().hit(r, t_min, t_max),
    //         Some(x) => {
    //             let hit_right = self.right.as_ref().unwrap().hit(r, t_min, x.t);
    //             match hit_right {
    //                 None => Some(x),
    //                 Some(y) => Some(y),
    //             }
    //         }
    //     }
    // }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.boxx.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = match self.left.as_ref() {
            Some(left) => left.hit(r, t_min, t_max),
            None => None,
        };
        let hit_right = match self.right.as_ref() {
            Some(right) => right.hit(r, t_min, t_max),
            None => None,
        };
        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }
}
