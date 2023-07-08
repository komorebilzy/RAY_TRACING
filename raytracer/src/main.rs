mod hittablelist;
use hittablelist::*;

mod hittable;
use hittable::*;

mod vect3;
use vect3::*;

mod ray;
use ray::*;

mod sphere;
use sphere::*;

mod rtweekend;
use rtweekend::*;

mod camear;
use camear::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use std::rc::Rc;
use std::{fs::File, process::exit};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vect3 {
    let mut rec = HitRecord::new();
    let infinity = f64::INFINITY;
    if world.hit(r, 0.0, infinity, &mut rec) {
        Vect3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5
    } else {
        let unit_direction: Vect3 = unit_vector(r.direction());
        let t: f64 = (unit_direction.y() + 1.0) * 0.5;
        Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let path = "output/book1/image6.jpg";

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vect3::new(0.0, 0.0, 0.0);
    let horizontal = Vect3::new(viewport_width, 0.0, 0.0);
    let vertical = Vect3::new(0.0, viewport_height, 0.0);
    let _lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vect3::new(0.0, 0.0, focal_length);

    let mut world = HitableList::new();
    world.add(Rc::new(Sphere::new(Vect3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vect3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    for j in 0..height {
        for i in 0..width {
            //获得(i,j)对应的（R,G,B）
            let pixel = img.get_pixel_mut(i, height - j - 1);
            let mut pixel_color = Vect3::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u: f64 = ((i as f64) + random_double()) / (width as f64);
                let v: f64 = ((j as f64) + random_double()) / (height as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            let scale: f64 = 1.0 / (samples_per_pixel as f64);
            let r: f64 = (256_f64) * clamp(scale * pixel_color[0], 0.0, 0.999);
            let g: f64 = (256_f64) * clamp(scale * pixel_color[1], 0.0, 0.999);
            let b: f64 = (256_f64) * clamp(scale * pixel_color[2], 0.0, 0.999);
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
