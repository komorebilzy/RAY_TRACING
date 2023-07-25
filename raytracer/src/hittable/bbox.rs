use crate::*;
pub struct MyBox {
    pub box_min: Vect3,
    pub box_max: Vect3,
    pub sides: HitableList,
}
impl MyBox {
    pub fn new<M: Material + Clone + 'static>(p0: Vect3, p1: Vect3, ptr: M) -> Self {
        let mut ans = MyBox {
            box_min: (p0),
            box_max: (p1),
            sides: (HitableList::new()),
        };
        ans.sides.add(Box::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        )));
        ans.sides.add(Box::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        )));
        ans.sides.add(Box::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        )));
        ans.sides.add(Box::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        )));
        ans.sides.add(Box::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        )));
        ans.sides.add(Box::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr,
        )));
        ans
    }
}
impl Hittable for MyBox {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(self.box_min, self.box_max);
        Some(output_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
