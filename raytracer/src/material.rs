use crate::*;
use std::{f64::consts::PI, option::Option};
pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: HitRecord, _pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Vect3) -> Vect3 {
        Vect3::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new1(a: Vect3) -> Self {
        Self {
            albedo: (Arc::new(SolidColor::new1(a))),
        }
    }
    // pub fn new2(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: (a) }
    // }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: HitRecord, pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        let mut uvw = Onb::default();
        uvw.build_from_w(rec.normal);
        let direction = uvw.local2(random_cosine_direction());
        let scattered = Ray::new(rec.p, unit_vector(direction), _r_in.time());
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        *pdf = dot(uvw.w(), scattered.direction()) / PI;
        let ans = Pair::new(alb, scattered);
        Some(ans)
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: HitRecord, _scattered: &Ray) -> f64 {
        let cosin = dot(_rec.normal, unit_vector(_scattered.direction()));
        if cosin < 0.0 {
            0.0
        } else {
            cosin / PI
        }
    }
}

pub struct Metal {
    pub albedo: Vect3,
    pub fuzz: f64,
}

impl Metal {
    // pub fn new(a: Vect3, f: f64) -> Self {
    //     Self {
    //         albedo: (a),
    //         fuzz: (if f < 1.0 { f } else { 1.0 }),
    //     }
    // }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: HitRecord, _pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        let attenuation = self.albedo;
        let ans = Pair::new(attenuation, scattered);

        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ans)
        } else {
            None
        }
    }
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    // pub fn new(index_of_refractiom: f64) -> Self {
    //     Self {
    //         ir: (index_of_refractiom),
    //     }
    // }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r = (1.0 - ref_idx) / (1.0 + ref_idx);
        r = r * r;
        r + (1.0 - r) * ((1.0 - cosine).powf(5.0))
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: HitRecord, _pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        let attenuation = Vect3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || material::Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.p, direction, r_in.time());
        let ans = Pair::new(attenuation, scattered);
        Some(ans)
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    // pub fn new1(a: Arc<Texture>) -> Self {
    //     Self { emit: (a) }
    // }
    pub fn new2(c: Vect3) -> Self {
        Self {
            emit: (Arc::new(SolidColor::new1(c))),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: HitRecord, _pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vect3) -> Vect3 {
        self.emit.value(_u, _v, _p)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}
impl Isotropic {
    // pub fn new1(c: Vect3) -> Self {
    //     Self {
    //         albedo: (Arc::new(SolidColor::new1(c))),
    //     }
    // }
    // pub fn new2(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: (a) }
    // }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: HitRecord, _pdf: &mut f64) -> Option<Pair<Vect3, Ray>> {
        Some(Pair {
            first: (self.albedo.value(rec.u, rec.v, rec.p)),
            second: (Ray::new(rec.p, random_in_unit_sphere(), r_in.time())),
        })
    }
}
