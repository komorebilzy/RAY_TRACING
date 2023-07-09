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

mod material;
use material::*;

mod pair;
use pair::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use std::rc::Rc;
use std::{fs::File, process::exit};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i64) -> Vect3 {
    if depth <= 0 {
        return Vect3::new(0.0, 0.0, 0.0);
    }
    let infinity = f64::INFINITY;
    let rec = world.hit(r, 0.001, infinity);
    match rec {
        Some(x) => match x.mat_ptr.scatter(*r, x.clone()) {
            Some(x) => x.first * ray_color(&x.second, world, depth - 1),
            None => Vect3::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction: Vect3 = unit_vector(r.direction());
            let t: f64 = (unit_direction.y() + 1.0) * 0.5;
            Vect3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vect3::new(0.5, 0.7, 1.0) * t
        }
    }
}
fn main() {
    let path = "output/book1/image18.jpg";

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
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
    let materail_ground = Rc::new(Lambertian::new(Vect3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vect3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Vect3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Rc::new(Sphere::new(
        Vect3::new(0.0, -100.5, -1.0),
        100.0,
        materail_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vect3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vect3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Vect3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vect3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = Camera::new(
        Vect3::new(-2.0, 2.0, 1.0),
        Vect3::new(0.0, 0.0, -1.0),
        Vect3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    );

    for j in 0..height {
        for i in 0..width {
            //获得(i,j)对应的（R,G,B）
            let pixel = img.get_pixel_mut(i, height - j - 1);
            let mut pixel_color = Vect3::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u: f64 = ((i as f64) + random_double()) / (width as f64);
                let v: f64 = ((j as f64) + random_double()) / (height as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            let scale: f64 = 1.0 / (samples_per_pixel as f64);
            let r: f64 = (scale * pixel_color[0]).sqrt();
            let g: f64 = (scale * pixel_color[1]).sqrt();
            let b: f64 = (scale * pixel_color[2]).sqrt();
            *pixel = image::Rgb([
                ((256_f64) * clamp(r, 0.0, 0.999)) as u8,
                ((256_f64) * clamp(g, 0.0, 0.999)) as u8,
                ((256_f64) * clamp(b, 0.0, 0.999)) as u8,
            ]);
            progress.inc(1);
        }
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
