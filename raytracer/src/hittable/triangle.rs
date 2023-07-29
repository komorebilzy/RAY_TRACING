// use crate::*;

// #[derive(Clone)]
// pub struct Triangle<M: Material> {
//     pub a: Vect3,
//     pub n: Vect3,
//     pub pc: Vect3,
//     pub pb: Vect3,
//     pub bbox: Aabb,
//     pub material: M,
// }

// impl<M: Material> Triangle<M> {
//     pub fn new(a: Vect3, b: Vect3, c: Vect3, material: M) -> Self {
//         let e1 = b - a;
//         let e2 = c - a;
//         let ne = cross(e1, e2);
//         let nel = ne.length();
//         let n = unit_vector(ne);
//         let mut min = Vect3::default();
//         let mut max = Vect3::default();
//         for i in 0..3 {
//             min[i] = a[i].min(b[i]).min(c[i]) - 0.0001;
//             max[i] = a[i].max(b[i]).max(c[i]) + 0.0001;
//         }
//         Self {
//             a: (a),
//             n: (n),
//             pc: (cross(e2, n) / nel),
//             pb: (cross(n, e1) / nel),
//             bbox: (Aabb::new(min, max)),
//             material: (material),
//         }
//     }
// }

// impl<M: Material> Hittable for Triangle<M> {
//     fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
//         let oa = _r.origin() - self.a;
//         let t = -dot(oa, self.n) / dot(_r.direction(), self.n);
//         if t < _t_min || t > _t_max {
//             None
//         } else {
//             let p = oa + _r.direction() * t;
//             let u = dot(self.pc, p);
//             let v = dot(self.pb, p);
//             if (0.0..=1.0).contains(&(u + v))
//                 && (0.0..=1.0).contains(&u)
//                 && (0.0..=1.0).contains(&v)
//             {
//                 let mut ans = HitRecord::new(t, p, &self.material);
//                 ans.u = u;
//                 ans.v = v;
//                 ans.normal = self.n;
//                 Some(ans)
//             } else {
//                 None
//             }
//         }
//     }
//     fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
//         Some(self.bbox.clone())
//     }
// }

use crate::*;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Triangle<M: Material> {
    pub a: Vect3,
    pub n: Vect3,
    pub pb: Vect3,
    pub pc: Vect3,
    //pc perpendicular to ac with length of ac/2*area
    pub bbox: Aabb,
    pub mat: M,
    pub uva: (f64, f64),
    pub uvab: (f64, f64),
    pub uvac: (f64, f64),
    //texture coordinate
}

impl<M: Material> Triangle<M> {
    pub fn new(
        a: &Vect3,
        b: &Vect3,
        c: &Vect3,
        mat: M,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
    ) -> Self {
        let ab = *b - *a;
        let ac = *c - *a;
        let normal = cross(ab, ac);
        let area2 = normal.length();
        let n = unit_vector(&normal);
        let mut min = Vect3::default();
        let mut max = Vect3::default();
        for i in 0..3 {
            min[i] = a[i].min(b[i]).min(c[i]) - 0.000001;
            max[i] = a[i].max(b[i]).max(c[i]) + 0.000001;
        }
        Self {
            a: *a,
            n,
            pb: cross(n, ab) / area2,
            pc: cross(ac, n) / area2,
            mat,
            bbox: Aabb::new(min, max),
            uva: (ua, va),
            uvab: (ub - ua, vb - va),
            uvac: (uc - ua, vc - va),
        }
    }

    //计算三角形面积
    pub fn area(&self) -> f64 {
        cross(self.pb, self.pc).length() / 2.0
    }

    //计算三角形边向量
    pub fn get_edges(&self) -> (Vect3, Vect3) {
        let area2 = self.area() * 2.0;
        let ab = cross(self.pb, self.n) * area2;
        let ac = cross(self.n, self.pc) * area2;

        let normal = cross(ab, ac);
        if unit_vector(&normal) == self.n {
            (ab, ac)
        } else {
            panic!("triangle get edges error")
        }
    }

    //根据u、v计算对应纹理值
    pub fn uv_coordinate(&self, u: f64, v: f64) -> (f64, f64) {
        (
            self.uva.0 + u * self.uvab.0 + v * self.uvac.0,
            self.uva.1 + u * self.uvab.1 + v * self.uvac.1,
        )
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oa = self.a - r.origin();
        let t = dot(&oa, &self.n) / dot(&r.direction(), &self.n);
        if t < t_min || t_max < t {
            return None;
        }
        let p = r.point_at_parameter(t);
        let ap = p - self.a;
        let u = dot(&ap, &self.pc);
        let v = dot(&ap, &self.pb);
        // AP = uAB + vAC
        if u >= 0. && v >= 0. && u + v <= 1. {
            let (x, y) = self.uv_coordinate(u, v);
            let rec = HitRecord {
                p,
                normal: self.n,
                t,
                u: x,
                v: y,
                front_face: true,
                mat_ptr: &self.mat,
            };
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
