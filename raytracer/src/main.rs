use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::io::{self, Read};
use std::mem::Discriminant;
// use std::mem::Discriminant;
use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};
use std::rc::Rc;
use std::vec;
use std::{fs::File, process::exit};
#[derive(Debug, Clone, Copy)]
#[warn(dead_code)]
struct Vect3 {
    e: [f64; 3],
}

impl Vect3 {
    // fn default() -> Self {
    //     Self { e: [0.0, 0.0, 0.0] }
    // }

    fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    fn x(&self) -> f64 {
        self.e[0]
    }
    fn y(&self) -> f64 {
        self.e[1]
    }
    fn z(&self) -> f64 {
        self.e[2]
    }
    // fn r(&self) -> f64 {
    //     self.e[0]
    // }
    // fn g(&self) -> f64 {
    //     self.e[1]
    // }
    // fn b(&self) -> f64 {
    //     self.e[2]
    // }
    fn length(&self) -> f64 {
        let sum_of_squres = self.e.iter().map(|x| x * x).sum::<f64>();
        sum_of_squres.sqrt()
    }
    // fn squared_length(&self) -> f64 {
    //     self.e.iter().map(|x| x * x).sum::<f64>()
    // }
    // fn make_unit_vector(&mut self) {
    //     let k = 1.0 / self.length();
    //     self.e[0] *= k;
    //     self.e[1] *= k;
    //     self.e[2] *= k;
    // }
}

impl ops::Neg for Vect3 {
    type Output = Vect3;
    fn neg(self) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = -self.e[i];
        }
        result
    }
}

impl ops::Index<usize> for Vect3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vect3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign<Vect3> for Vect3 {
    fn add_assign(&mut self, rhs: Vect3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl SubAssign<Vect3> for Vect3 {
    fn sub_assign(&mut self, rhs: Vect3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl MulAssign<Vect3> for Vect3 {
    fn mul_assign(&mut self, rhs: Vect3) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}
impl MulAssign<f64> for Vect3 {
    fn mul_assign(&mut self, scalar: f64) {
        self[0] *= scalar;
        self[1] *= scalar;
        self[2] *= scalar;
    }
}
impl DivAssign<Vect3> for Vect3 {
    fn div_assign(&mut self, rhs: Vect3) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}
impl DivAssign<f64> for Vect3 {
    fn div_assign(&mut self, scalar: f64) {
        self[0] /= scalar;
        self[1] /= scalar;
        self[2] /= scalar;
    }
}

impl ops::Add<Vect3> for Vect3 {
    type Output = Vect3;
    fn add(self, rhs: Vect3) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] + rhs.e[i];
        }
        result
    }
}

impl ops::Sub<Vect3> for Vect3 {
    type Output = Vect3;
    fn sub(self, rhs: Vect3) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] - rhs.e[i];
        }
        result
    }
}

impl ops::Mul<Vect3> for Vect3 {
    type Output = Vect3;
    fn mul(self, rhs: Vect3) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] * rhs.e[i];
        }
        result
    }
}
impl ops::Mul<f64> for Vect3 {
    type Output = Vect3;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] * rhs;
        }
        result
    }
}
impl ops::Div<Vect3> for Vect3 {
    type Output = Vect3;
    fn div(self, rhs: Vect3) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] / rhs.e[i];
        }
        result
    }
}
impl ops::Div<f64> for Vect3 {
    type Output = Vect3;
    fn div(self, rhs: f64) -> Self::Output {
        let mut result = Vect3 { e: [0.0; 3] };
        for i in 0..3 {
            result.e[i] = self.e[i] / rhs;
        }
        result
    }
}

//to be learned
impl Read for Vect3 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let input = io::stdin().read(buf)?;
        let input_str = String::from_utf8_lossy(&buf[..input]).trim().to_string();
        let values: Vec<f64> = input_str
            .split_whitespace()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .collect();
        if values.len() == 3 {
            self.e.copy_from_slice(&values[..]);
            Ok(input)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid number of values",
            ))
        }
    }
}
fn unit_vector(v: Vect3) -> Vect3 {
    v / v.length()
}
fn dot(v1: Vect3, v2: Vect3) -> f64 {
    let result: f64 = v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2];
    result
}

// #[derive(Clone, Copy)]
struct Ray {
    a: Vect3,
    b: Vect3,
}
impl Ray {
    // fn default() -> Self {
    //     Self {
    //         a: (Vect3::default()),
    //         b: (Vect3::default()),
    //     }
    // }
    fn new(a: Vect3, b: Vect3) -> Self {
        Self { a: (a), b: (b) }
    }
    fn origin(&self) -> Vect3 {
        self.a
    }
    fn direction(&self) -> Vect3 {
        self.b
    }
    fn point_at_parameter(self, t: f64) -> Vect3 {
        self.a + self.b * t
    }
}

pub struct HitRecord {
    t: f64,
    p: Vect3,
    normal: Vect3,
    front_face: bool,
}
impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vect3) {
        self.front_face = dot(r.direction(), outward_normal) > 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

trait HITTABLE {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
    }
}

pub struct Sphere {
    center: Vect3,
    radius: f64,
}
impl Sphere {
    fn new(cen: Vect3, r: f64) -> Self {
        Self {
            center: (cen),
            radius: (r),
        }
    }
}
impl HITTABLE for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vect3 = r.origin() - self.center;
        let a: f64 = dot(r.direction(), r.direction());
        let b: f64 = dot(oc, r.direction());
        let c: f64 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut temp: f64 = (-b - (b * b - a * c).sqrt()) / a;
        if temp > t_max || temp < t_min {
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp > t_max || temp < t_min {
                return false;
            }
        }
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        // rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal: Vect3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}

// #[derive(Clone)]
pub struct HitableList {
    objects: Vec<Rc<dyn HITTABLE>>,
}
impl HitableList {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    fn add(&mut self, object: Rc<dyn HITTABLE>) {
        self.objects.push(object)
    }
}
impl HITTABLE for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord;
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in self.objects.clone() {
            if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = &mut temp_rec;
            }
        }
        hit_anything
    }
}

fn hit_sphere(center: Vect3, radius: f64, r: &Ray) -> f64 {
    let oc: Vect3 = r.origin() - center;
    let a: f64 = dot(r.direction(), r.direction());
    let b: f64 = dot(oc, r.direction()) * 2.0;
    let c: f64 = dot(oc, oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant).sqrt() / (a * 2.0)
    }
}

fn color(r: &Ray, world: Rc<dyn HITTABLE>) -> Vect3 {
    let rec: HitRecord;
    let infinity = f64::INFINITY;
    if world.hit(r, 0.0, infinity, &mut rec) {
        return Vect3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5;
    } else {
        let unit_direction: Vect3 = unit_vector(r.direction());
        let t: f64 = (unit_direction.y() + 1.0) * 0.5;
        return Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t;
    }
    // let t: f64 = hit_sphere(Vect3::new(0.0, 0.0, -1.0), 0.5, &r);
    // if t > 0.0 {
    //     let a = Vect3::new(0.0, 0.0, -1.0);
    //     let n: Vect3 = unit_vector(r.point_at_parameter(t) - a);
    //     Vect3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    // } else {
    //     let unit_direction: Vect3 = unit_vector(r.direction());
    //     let t: f64 = unit_direction.y() * 0.5 + 1.0;
    //     let result: Vect3 = Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t;
    //     result
    // }
    // let unit_direction = unit_vector(r.direction());
    // let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    // let result = Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t;
    // result
}

fn main() {
    let path = "output/book1/image5.jpg";

    let width = 200;
    let height = 100;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let lower_left_corner = Vect3::new(-2.0, -1.0, -1.0);
    let horizontal = Vect3::new(4.0, 0.0, 0.0);
    let vertical = Vect3::new(0.0, 2.0, 0.0);
    let origin = Vect3::new(0.0, 0.0, 0.0);
    let world: &mut HitableList;
    world.add(Rc::new(Sphere::new(Vect3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vect3::new(0.0, -100.5, -1.0), 100.0)));
    // let list: [Box<dyn HITTABLE>; 2];
    // list[0] = Box::new(Sphere::new(Vect3::new(0.0, 0.0, -1.0), 0.5));
    // list[1] = Box::new(Sphere::new(Vect3::new(0.0, -100.5, -1.0), 100.0));
    // let world: Box<dyn HITTABLE> = Box::new(HitableList::new());
    let world2 = Rc::new(world);
    for j in 0..height {
        for i in 0..width {
            //获得(i,j)对应的（R,G,B）
            let pixel = img.get_pixel_mut(i, height - j - 1);

            let u: f64 = (i as f64) / (width as f64);
            let v: f64 = (j as f64) / (height as f64);
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vect3 = color(&r, world2);
            let r: f64 = 255.999 * col[0];
            let g: f64 = 255.999 * col[1];
            let b: f64 = 255.999 * col[2];
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
    exit(0);
}
