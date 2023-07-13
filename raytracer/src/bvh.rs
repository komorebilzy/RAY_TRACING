// use image::flat::SampleLayout;

use crate::*;
pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub boxx: Aabb,
}

pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: i64) -> std::cmp::Ordering {
    match a.bounding_box(0.0, 0.0) {
        None => {
            eprintln!("No bounding box in bvh_node constructor");
            std::cmp::Ordering::Equal
        }
        Some(x) => match b.bounding_box(0.0, 0.0) {
            None => {
                eprintln!("No bounding box in bvh_node constructor");
                std::cmp::Ordering::Equal
            }
            Some(y) =>{
                if  x.min().e[axis as usize]<y.min().e[axis as usize] {std::cmp::Ordering::Less}
                else {std::cmp::Ordering::Greater}
            }
                // x.min().e[axis as usize]
                // .partial_cmp(&y.min().e[axis as usize])
                // .unwrap_or(std::cmp::Ordering::Equal),
        },
    }
}

pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn prinew(
        src_objects: Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut ans = BvhNode {
            left: (src_objects[0].clone()),
            right: (src_objects[0].clone()),
            boxx: (Aabb::default()),
        };
        let mut objects = src_objects;
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
            ans.left = objects[start].clone();
            ans.right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start].clone(), &objects[start + 1].clone())
                == std::cmp::Ordering::Less
            {
                ans.left = objects[start].clone();
                ans.right = objects[start + 1].clone();
            } else {
                ans.left = objects[start + 1].clone();
                ans.right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            ans.left = Rc::new(BvhNode::prinew(objects.clone(), start, mid, time0, time1));
            ans.left = Rc::new(BvhNode::prinew(objects, mid, end, time0, time1));
        }
        let x = ans.left.bounding_box(time0, time1).unwrap();
        let y = ans.right.bounding_box(time0, time1).unwrap();
        ans.boxx = surrounding_box(x, y);
        ans
    }

    pub fn new(list: HitableList, time0: f64, time1: f64) -> Self {
        BvhNode::prinew(
            list.objects.clone(),
            0_usize,
            list.objects.clone().len(),
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
        if !self.boxx.hit(*r, t_min, t_max) {
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
