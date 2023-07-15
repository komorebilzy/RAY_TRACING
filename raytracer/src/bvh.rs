// use image::flat::SampleLayout;

use crate::*;
#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub boxx: Aabb,
}

pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i64) -> std::cmp::Ordering {
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

pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn prinew(
        src_objects: &mut [Arc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut ans = BvhNode {
            left: (src_objects[start].clone()),
            right: (src_objects[start].clone()),
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
        if object_span == 2 {
            if comparator(&src_objects[start].clone(), &src_objects[start + 1].clone())
                == std::cmp::Ordering::Less
            {
                ans.left = src_objects[start].clone();
                ans.right = src_objects[start + 1].clone();
            } else {
                ans.left = src_objects[start + 1].clone();
                ans.right = src_objects[start].clone();
            }
        } else if object_span != 1 {
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            ans.left = Arc::new(BvhNode::prinew(src_objects, start, mid, time0, time1));
            ans.right = Arc::new(BvhNode::prinew(src_objects, mid, end, time0, time1));
        }
        let left = ans.left.bounding_box(time0, time1);
        let right = ans.right.bounding_box(time0, time1);
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

    pub fn new(list: HitableList, time0: f64, time1: f64) -> Self {
        BvhNode::prinew(
            &mut list.objects.clone(),
            0_usize,
            list.objects.len(),
            time0,
            time1,
        )
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.boxx.clone())
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.boxx.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        match hit_left {
            None => self.right.hit(r, t_min, t_max),
            Some(x) => {
                let hit_right = self.right.hit(r, t_min, x.t);
                match hit_right {
                    None => Some(x),
                    Some(y) => Some(y),
                }
            }
        }
    }
}
