use crate::*;
use std::{f64::consts::PI, option::Option};
#[derive(Clone)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vect3,
    pub pdf_ptr: Option<Box<CosinePdf>>,
}
pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Vect3) -> Vect3 {
        Vect3::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone, Copy)]
pub struct EmptyMaterial {}
impl Material for EmptyMaterial {}
pub const DEFAULT_MATERIAL: EmptyMaterial = EmptyMaterial {};

#[derive(Clone, Default)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}
impl Lambertian<SolidColor> {
    pub fn new1(a: Vect3) -> Self {
        Self {
            albedo: SolidColor::new1(a),
        }
    }
}
impl<T: Texture> Lambertian<T> {
    pub fn new2(a: T) -> Self {
        Self { albedo: (a) }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ans = ScatterRecord {
            specular_ray: (*_r_in),
            is_specular: (false),
            attenuation: (self.albedo.value(rec.u, rec.v, rec.p)),
            pdf_ptr: Some(Box::new(CosinePdf::new(rec.normal))),
        };
        Some(ans)
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
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
    pub fn new(a: Vect3, f: f64) -> Self {
        Self {
            albedo: (a),
            fuzz: (if f < 1.0 { f } else { 1.0 }),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        let ans = ScatterRecord {
            specular_ray: scattered,
            attenuation: self.albedo,
            is_specular: true,
            pdf_ptr: None,
        };
        Some(ans)
    }
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn new(index_of_refractiom: f64) -> Self {
        Self {
            ir: (index_of_refractiom),
        }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r = (1.0 - ref_idx) / (1.0 + ref_idx);
        r = r * r;
        r + (1.0 - r) * ((1.0 - cosine).powf(5.0))
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
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
        let ans = ScatterRecord {
            specular_ray: (scattered),
            is_specular: (true),
            attenuation: (Vect3::new(1.0, 1.0, 1.0)),
            pdf_ptr: None,
        };
        Some(ans)
    }
}

pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

// impl<T: Texture> DiffuseLight<T> {
//     pub fn new1(a: T) -> Self {
//         Self { emit: (a) }
//     }
// }
impl DiffuseLight<SolidColor> {
    pub fn new2(c: Vect3) -> Self {
        Self {
            emit: SolidColor::new1(c),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Vect3) -> Vect3 {
        // self.emit.value(_u, _v, _p)
        if _rec.front_face {
            self.emit.value(_u, _v, _p)
        } else {
            Vect3::new(0.0, 0.0, 0.0)
        }
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}
impl Isotropic<SolidColor> {
    pub fn new1(c: Vect3) -> Self {
        Self {
            albedo: SolidColor::new1(c),
        }
    }
    // pub fn new2(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: (a) }
    // }
}
impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ans = ScatterRecord {
            specular_ray: (Ray::new(rec.p, random_in_unit_sphere(), r_in.time())),
            is_specular: (true),
            attenuation: (self.albedo.value(rec.u, rec.v, rec.p)),
            pdf_ptr: None,
        };
        Some(ans)
    }
}
