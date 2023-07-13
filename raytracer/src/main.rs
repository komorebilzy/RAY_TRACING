mod hittablelist;
use hittablelist::*;

mod hittable;
use hittable::*;

mod vect3;
use vect3::*;

mod ray;
use ray::*;

mod sphere;
// use sphere::*;

mod rtweekend;
use rtweekend::*;

mod camear;
use camear::*;

mod material;
use material::*;

mod pair;
use pair::*;

// mod moving_sphere;
// use moving_sphere::*;

mod aabb;
use aabb::*;

// mod bvh;
// use bvh::*;

mod texture;
use texture::*;

mod perlin;
use perlin::*;

mod aarect;
use aarect::*;

mod bbox;
use bbox::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use std::rc::Rc;
use std::{fs::File, process::exit};

// fn random_scene() -> HitableList {
//     let mut world = HitableList::new();
//     let checker = Rc::new(CheckerTexture::new2(
//         Vect3::new(0.2, 0.3, 0.1),
//         Vect3::new(0.9, 0.9, 0.9),
//     ));
//     // let ground_material = Rc::new(Lambertian::new(Vect3::new(0.5, 0.5, 0.5)));
//     world.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Rc::new(Lambertian::new2(checker)),
//     )));
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_double();
//             let center = Vect3::new(
//                 a as f64 + random_double() * 0.9,
//                 0.2,
//                 b as f64 + 0.9 * random_double(),
//             );

//             if (center - Vect3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 let sphere_material: Rc<dyn Material>;
//                 if choose_mat < 0.8 {
//                     let albedo = Vect3::random() * Vect3::random();
//                     sphere_material = Rc::new(Lambertian::new1(albedo));
//                     let center2 = center + Vect3::new(0.0, random_double_rng(0.0, 0.5), 0.0);
//                     world.add(Rc::new(MovingSphere::new(
//                         center,
//                         center2,
//                         0.0,
//                         1.0,
//                         0.2,
//                         sphere_material,
//                     )));
//                 } else if choose_mat < 0.95 {
//                     let albedo = Vect3::random1(0.5, 1.0);
//                     let fuzz = random_double_rng(0.0, 0.5);
//                     sphere_material = Rc::new(Metal::new(albedo, fuzz));
//                     world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
//                 } else {
//                     sphere_material = Rc::new(Dielectric::new(1.5));
//                     world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
//                 }
//             }
//         }
//     }
//     let material1 = Rc::new(Dielectric::new(1.5));
//     world.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, 1.0, 0.0),
//         1.0,
//         material1,
//     )));
//     let material2 = Rc::new(Lambertian::new1(Vect3::new(0.4, 0.2, 0.1)));
//     world.add(Rc::new(Sphere::new(
//         Vect3::new(-4.0, 1.0, 0.0),
//         1.0,
//         material2,
//     )));
//     let material3 = Rc::new(Metal::new(Vect3::new(0.7, 0.6, 0.5), 0.0));
//     world.add(Rc::new(Sphere::new(
//         Vect3::new(4.0, 1.0, 0.0),
//         1.0,
//         material3,
//     )));
//     world
// }

// fn two_spheres() -> HitableList {
//     let mut objects = HitableList::new();
//     let checker = Rc::new(CheckerTexture::new2(
//         Vect3::new(0.2, 0.3, 0.1),
//         Vect3::new(0.9, 0.9, 0.9),
//     ));
//     objects.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, -10.0, 0.0),
//         10.0,
//         Rc::new(Lambertian::new2(checker.clone())),
//     )));
//     objects.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, 10.0, 0.0),
//         10.0,
//         Rc::new(Lambertian::new2(checker)),
//     )));
//     objects
// }

// fn two_perlin_spheres() -> HitableList {
//     let mut objects = HitableList::new();
//     let pertext = Rc::new(NoiseTexture::new2(4.0));

// objects.add(Rc::new(Sphere::new(
//     Vect3::new(0.0, -1000.0, 0.0),
//     1000.0,
//     Rc::new(Lambertian::new2(pertext.clone())),
// )));
// objects.add(Rc::new(Sphere::new(
//     Vect3::new(0.0, 2.0, 0.0),
//     2.0,
//     Rc::new(Lambertian::new2(pertext)),
// )));
//     objects
// }
// fn earth() -> HitableList {
//     let earth_texture = Rc::new(ImageTexture::new("input/earthmap.jpg"));
//     let earth_surface = Rc::new(Lambertian::new2(earth_texture));
//     let globe = Rc::new(Sphere::new(Vect3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
//     let mut ans = HitableList::new();
//     ans.add(globe);
//     ans
// }
// fn simple_light() -> HitableList {
//     let mut objects = HitableList::new();
//     let pertext = Rc::new(NoiseTexture::new2(4.0));
//     objects.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Rc::new(Lambertian::new2(pertext.clone())),
//     )));
//     objects.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, 2.0, 0.0),
//         2.0,
//         Rc::new(Lambertian::new2(pertext)),
//     )));
//     let diffliight = Rc::new(DiffuseLight::new2(Vect3::new(4.0, 4.0, 4.0)));
//     objects.add(Rc::new(XyRect::new(
//         3.0,
//         5.0,
//         1.0,
//         3.0,
//         -2.0,
//         diffliight.clone(),
//     )));
//     objects.add(Rc::new(Sphere::new(
//         Vect3::new(0.0, 7.0, 0.0),
//         2.0,
//         diffliight,
//     )));
//     objects
// }
fn cornell_box() -> HitableList {
    let mut objects = HitableList::new();
    let red = Rc::new(Lambertian::new1(Vect3::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new1(Vect3::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new1(Vect3::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new2(Vect3::new(15.0, 15.0, 15.0)));
    objects.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Rc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Rc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Rc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Rc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Rc::new(Box::new(
        Vect3::new(130.0, 0.0, 65.0),
        Vect3::new(295.0, 165.0, 230.0),
        white.clone(),
    )));
    objects.add(Rc::new(Box::new(
        Vect3::new(265.0, 0.0, 295.0),
        Vect3::new(430.0, 330.0, 460.0),
        white,
    )));
    objects
}

fn ray_color(r: &Ray, background: Vect3, world: &dyn Hittable, depth: i64) -> Vect3 {
    if depth <= 0 {
        return Vect3::new(0.0, 0.0, 0.0);
    }
    let infinity = f64::INFINITY;
    let rec = world.hit(r, 0.001, infinity);
    match rec {
        Some(x) => match x.mat_ptr.scatter(*r, x.clone()) {
            Some(y) => {
                x.mat_ptr.emitted(x.u, x.v, x.p)
                    + y.first * ray_color(&y.second, background, world, depth - 1)
            }
            None => x.mat_ptr.emitted(x.u, x.v, x.p),
        },
        None => background,
    }
}
fn main() {
    let path = "output/book2/image18.jpg";

    let aspect_ratio = 1.0;
    let width = 600;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 200;
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

    let world = cornell_box();
    let lookfrom = Vect3::new(278.0, 278.0, -800.0);
    let lookat = Vect3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let background = Vect3::new(0.0, 0.0, 0.0);
    let vup = Vect3::new(0.0, 1.0, 0.0);
    let dis_to_focus = 10.0;

    // let world = random_scene();

    // let lookfrom = Vect3::new(13.0, 2.0, 3.0);
    // let lookat = Vect3::new(0.0, 0.0, 0.0);
    // let vup = Vect3::new(0.0, 1.0, 0.0);
    // let dis_to_focus = 10.0;
    // let aperture = 0.1;

    let cam = Camera::new(
        (lookfrom, lookat),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dis_to_focus,
        (0.0, 1.0),
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
                pixel_color += ray_color(&r, background, &world, max_depth);
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
