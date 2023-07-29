pub mod aarect;
pub mod bbox;
pub mod constant_medium;
pub mod hittablelist;
pub mod moving_sphere;
pub mod sphere;
pub mod triangle;

pub use aarect::*;
pub use bbox::*;
pub use constant_medium::*;
pub use hittablelist::*;
pub use moving_sphere::*;
pub use sphere::*;
pub use triangle::*;

use crate::*;
#[derive(Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vect3,
    pub normal: Vect3,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}
impl<'a> HitRecord<'a> {
    pub fn new(tt: f64, pp: Vect3, m: &'a dyn Material) -> Self {
        Self {
            t: (tt),
            p: (pp),
            normal: (Vect3::default()),
            front_face: (false),
            mat_ptr: (m),
            u: (0.0),
            v: (0.0),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vect3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _o: &Vect3, _v: &Vect3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vect3) -> Vect3 {
        Vect3::new(1.0, 0.0, 0.0)
    }
}

//平移
pub struct Translate<H: Hittable> {
    pub ptr: H,
    pub offset: Vect3,
}
impl<H: Hittable> Translate<H> {
    pub fn new(p: H, displacement: Vect3) -> Self {
        Self {
            ptr: (p),
            offset: (displacement),
        }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(&(r.origin() - self.offset), &r.direction(), r.time());
        match self.ptr.hit(&moved_r, t_min, t_max) {
            Some(x) => {
                let mut rec = x;
                rec.p += self.offset;
                // rec.set_face_normal(&moved_r, rec.normal);
                // rec.normal=
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr
            .bounding_box(time0, time1)
            .map(|x| Aabb::new(x.min() + self.offset, x.max() + self.offset))
        // match self.ptr.bounding_box(time0, time1) {
        //     Some(x) => Some(Aabb::new(x.min() + self.offset, x.max() + self.offset)),
        //     None => None,
        // }
    }
}

//绕y轴旋转
pub struct RotateY<H: Hittable> {
    ptr: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(p: H, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let infinity = f64::INFINITY;
        let mut min = Vect3::new(infinity, infinity, infinity);
        let mut max = Vect3::new(-infinity, -infinity, -infinity);
        let bbox = p.bounding_box(0.0, 1.0).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1.0 - i as f64) * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1.0 - j as f64) * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1.0 - i as f64) * bbox.min().z();
                    let newx = radians.cos() * x + radians.sin() * z;
                    let newz = -radians.sin() * x + radians.cos() * z;
                    let tester = Vect3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let ans = Some(Aabb::new(min, max));
        Self {
            ptr: (p),
            sin_theta: (radians.sin()),
            cos_theta: (radians.cos()),
            bbox: (ans),
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());
        match self.ptr.hit(&rotated_r, t_min, t_max) {
            Some(x) => {
                let mut p = x.p;
                let mut normal = x.normal;
                p[0] = self.cos_theta * x.p[0] + self.sin_theta * x.p[2];
                p[2] = -self.sin_theta * x.p[0] + self.cos_theta * x.p[2];
                normal[0] = self.cos_theta * x.normal[0] + self.sin_theta * x.normal[2];
                normal[2] = -self.sin_theta * x.normal[0] + self.cos_theta * x.normal[2];
                let mut ans = x;
                ans.p = p;
                // ans.set_face_normal(&rotated_r, normal);
                ans.normal = normal;
                Some(ans)
            }
            None => None,
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ans = (self.bbox).as_ref().unwrap();
        let anss = (*ans).clone();
        Some(anss)
    }
}

#[derive(Clone, Default)]
pub struct RotateX<H: Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl<H: Hittable> Hittable for RotateX<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = rotate_vec_x(r.origin_borrow(), self.sin_theta, self.cos_theta);
        let direction = rotate_vec_x(r.direction_borrow(), self.sin_theta, self.cos_theta);
        let rotated_r = Ray::new(&origin, &direction, r.time());

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let p = rotate_vec_x(&rec.p, -self.sin_theta, self.cos_theta);
            let normal = rotate_vec_x(&rec.normal, -self.sin_theta, self.cos_theta);
            rec.p = p;
            rec.normal = normal;
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let rotated_o = rotate_vec_x(o, self.sin_theta, self.cos_theta);
        let rotated_v = rotate_vec_x(v, self.sin_theta, self.cos_theta);
        self.ptr.pdf_value(&rotated_o, &rotated_v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let rotated_o = rotate_vec_x(o, self.sin_theta, self.cos_theta);
        let rotated_rand = self.ptr.random(&rotated_o);
        rotate_vec_x(&rotated_rand, -self.sin_theta, self.cos_theta)
    }
}

impl<H: Hittable> RotateX<H> {
    pub fn new(p: H, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0);
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let x = i * bbox.max().x() + (1.0 - i) * bbox.min().x();
                    let y = j * bbox.max().y() + (1.0 - j) * bbox.min().y();
                    let z = k * bbox.max().z() + (1.0 - k) * bbox.min().z();
                    let pre = Vec3::new(x, y, z);
                    let tester = rotate_vec_x(&pre, -sin_theta, cos_theta);
                    for c in 0..3 {
                        min[c] = tester[c].min(min[c]);
                        max[c] = tester[c].max(max[c]);
                    }
                }
            }
        }
        bbox = AABB::new(&min, &max);
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox: false,
            bbox: bbox.unwrap(),
        }
    }
}

#[derive(Clone, Default)]
pub struct RotateZ<H: Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl<H: Hittable> Hittable for RotateZ<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = rotate_vec_z(r.origin_borrow(), self.sin_theta, self.cos_theta);
        let direction = rotate_vec_z(r.direction_borrow(), self.sin_theta, self.cos_theta);

        let rotated_r = Ray::new(&origin, &direction, r.time());

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let p = rotate_vec_z(&rec.p, -self.sin_theta, self.cos_theta);
            let normal = rotate_vec_z(&rec.normal, -self.sin_theta, self.cos_theta);
            rec.p = p;
            rec.normal = normal;
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let rotated_o = rotate_vec_z(o, self.sin_theta, self.cos_theta);
        let rotated_v = rotate_vec_z(v, self.sin_theta, self.cos_theta);
        self.ptr.pdf_value(&rotated_o, &rotated_v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let rotated_o = rotate_vec_z(o, self.sin_theta, self.cos_theta);
        let rotated_rand = self.ptr.random(&rotated_o);
        rotate_vec_z(&rotated_rand, -self.sin_theta, self.cos_theta)
    }
}

impl<H: Hittable> RotateZ<H> {
    pub fn new(p: H, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0);
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let x = i * bbox.max().x() + (1.0 - i) * bbox.min().x();
                    let y = j * bbox.max().y() + (1.0 - j) * bbox.min().y();
                    let z = k * bbox.max().z() + (1.0 - k) * bbox.min().z();
                    let pre = Vec3::new(x, y, z);
                    let tester = rotate_vec_z(&pre, -sin_theta, cos_theta);
                    for c in 0..3 {
                        min[c] = tester[c].min(min[c]);
                        max[c] = tester[c].max(max[c]);
                    }
                }
            }
        }
        bbox = AABB::new(&min, &max);
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

//物体的前后面翻转
pub struct FlipFace<H: Hittable> {
    pub ptr: H,
}
impl<H: Hittable> FlipFace<H> {
    pub fn new(p: H) -> Self {
        Self { ptr: (p) }
    }
}
impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        match self.ptr.hit(_r, _t_min, _t_max) {
            Some(x) => {
                let mut ans = x.clone();
                ans.front_face = !x.front_face;
                Some(ans)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}
