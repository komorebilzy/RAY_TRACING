use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::io::{self, Read};
// use std::mem::Discriminant;
use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};
// use std::vec;
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

    // fn x(&self) -> f64 {
    //     self.e[0]
    // }
    fn y(&self) -> f64 {
        self.e[1]
    }
    // fn z(&self) -> f64 {
    //     self.e[2]
    // }
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
            result.e[i] = self.e[i] * rhs.e[i];
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
    // fn point_at_parameter(self, t: f64) -> Vect3 {
    //     self.a + self.b * t
    // }
}

fn hit_sphere(center: Vect3, radius: f64, r: &Ray) -> bool {
    let oc: Vect3 = r.origin() - center;
    let a: f64 = dot(r.direction(), r.direction());
    let b: f64 = dot(oc, r.direction())*2.0;
    let c: f64 = dot(oc, oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    discriminant > 0.0
}

fn color(r: Ray) -> Vect3 {
    if hit_sphere(Vect3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Vect3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction: Vect3 = unit_vector(r.direction());
        let t: f64 = unit_direction.y() * 0.5 + 1.0;
        let result: Vect3 = Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t;
        result
    }
    // let unit_direction = unit_vector(r.direction());
    // let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    // let result = Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t;
    // result
}

fn main() {
    let path = "output/book1/image3.jpg";

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
    for j in (0..height).rev() {
        for i in 0..width {
            //获得(i,j)对应的（R,G,B）
            let pixel = img.get_pixel_mut(i, j);

            let u: f64 = (i as f64) / (width as f64);
            let v: f64 = (j as f64) / (height as f64);
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vect3 = color(r);
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
