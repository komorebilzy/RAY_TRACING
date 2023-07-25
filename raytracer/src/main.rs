mod hittable;
use hittable::*;

mod utility;
use utility::*;

mod pdf;
use pdf::*;

mod camear;
use camear::*;

mod material;
use material::*;

mod aabb;
use aabb::*;

mod bvh;
use bvh::*;

mod texture;
use texture::*;

mod perlin;
use perlin::*;

mod color;
use color::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use rand::Rng;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::{fs::File, process::exit};

fn final_scene() -> HitableList {
    let mut boxes1 = HitableList::new();
    let ground = Lambertian::new1(Vect3::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_rng(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Box::new(MyBox::new(
                Vect3::new(x0, y0, z0),
                Vect3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects = HitableList::new();
    objects.add(Box::new(BvhNode::new(boxes1, 0.0, 1.0)));

    // objects.add(Rc::new(BvhNode::new(boxes1, 0.0, 1.0)));
    let light = DiffuseLight::new2(Vect3::new(7.0, 7.0, 7.0));
    objects.add(Box::new(FlipFace::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    ))));
    let center1 = Vect3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vect3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new1(Vect3::new(0.7, 0.3, 0.1));
    objects.add(Box::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    objects.add(Box::new(Sphere::new(
        Vect3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(01.5),
    )));
    objects.add(Box::new(Sphere::new(
        Vect3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Vect3::new(0.8, 0.8, 0.9), 1.0),
    )));
    let mut boundary = Sphere::new(Vect3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(Box::new(boundary.clone()));
    objects.add(Box::new(ConstantMedium::new2(
        boundary.clone(),
        0.2,
        Vect3::new(0.2, 0.4, 0.9),
    )));
    boundary = Sphere::new(Vect3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    objects.add(Box::new(ConstantMedium::new2(
        boundary,
        0.0001,
        Vect3::new(1.0, 1.0, 1.0),
    )));
    let emat = Lambertian::new2(ImageTexture::new("input/earthmap.jpg"));
    objects.add(Box::new(Sphere::new(
        Vect3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new2(0.1);
    objects.add(Box::new(Sphere::new(
        Vect3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new2(pertext),
    )));
    let mut boxes2 = HitableList::new();
    let white = Lambertian::new1(Vect3::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            Vect3::random1(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    objects.add(Box::new(Translate::new(
        RotateY::new(BvhNode::new(boxes2, 0.0, 1.0), 15.0),
        Vect3::new(-100.0, 270.0, 395.0),
    )));
    objects
}

fn ray_color(
    r: &Ray,
    background: Vect3,
    world: &impl Hittable,
    lights: &impl Hittable,
    depth: i64,
) -> Vect3 {
    if depth <= 0 {
        return Vect3::new(0.0, 0.0, 0.0);
    }
    let infinity = f64::INFINITY;
    let mut _pdf_val = 0.0;
    let rec = world.hit(r, 0.001, infinity);
    match rec {
        Some(x) => {
            let emitted = x.mat_ptr.emitted(r, &x, x.u, x.v, x.p);
            match x.mat_ptr.scatter(r, &x) {
                Some(y) => {
                    if y.is_specular {
                        return y.attenuation
                            * ray_color(&y.specular_ray, background, world, lights, depth - 1);
                    }
                    let light_ptr = HittablePdf::new(lights, x.p);
                    let tmp = y.pdf_ptr.unwrap();
                    let p = MixturePdf::new(&light_ptr, tmp.as_ref());
                    let scattered = Ray::new(x.p, p.generate(), r.time());
                    _pdf_val = p.value(scattered.direction());
                    emitted
                        + y.attenuation
                            * x.mat_ptr.scattering_pdf(r, &x, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            / _pdf_val
                }
                None => emitted,
            }
        }
        None => background,
    }
}
fn main() {
    let path = std::path::Path::new("output/book2/image_final.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    let aspect_ratio = 1.0;
    let width = 600;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 1000;
    let max_depth = 50;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vect3::new(0.0, 0.0, 0.0);
    let horizontal = Vect3::new(viewport_width, 0.0, 0.0);
    let vertical = Vect3::new(0.0, viewport_height, 0.0);
    let _lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vect3::new(0.0, 0.0, focal_length);

    let world = final_scene();
    let lights = XzRect::new(123.0, 423.0, 147.0, 412.0, 554.0, DEFAULT_MATERIAL);

    let lookfrom = Vect3::new(478.0, 278.0, -600.0);
    let lookat = Vect3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let background = Vect3::new(0.0, 0.0, 0.0);
    let vup = Vect3::new(0.0, 1.0, 0.0);
    let dis_to_focus = 10.0;

    let cam = Camera::new(
        (lookfrom, lookat),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dis_to_focus,
        (0.0, 1.0),
    );

    const THREAD_NUM: usize = 10;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let mut recv: Vec<_> = Vec::new();
    let mut pixel_list: Vec<Vec<_>> = Vec::new();
    for _k in 0..THREAD_NUM {
        pixel_list.push(Vec::new());
    }
    for j in 0..height {
        for i in 0..width {
            let mut rng = rand::thread_rng();
            let id = rng.gen_range(0..THREAD_NUM);
            pixel_list[id].push(Position::pos(j, i));
        }
    }
    let multi_progress = MultiProgress::new();
    let world = Arc::new(world);
    let lights = Arc::new(lights);

    for (k, pixel_pos) in pixel_list.iter().enumerate().take(THREAD_NUM) {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let _world = world.clone();
        let _cam = cam.clone();
        let _pixel_pos = pixel_pos.clone();
        let _lights = lights.clone();
        let pb = multi_progress.add(ProgressBar::new((_pixel_pos.len() / width as usize) as u64));
        pb.set_prefix(format!("Process {}", k));
        let handle = thread::spawn(move || {
            let mut color_list: Vec<(Position, Vect3)> = Vec::new();
            let mut num = 0;
            for pixel in _pixel_pos {
                let mut pixel_color = Vect3::new(0.0, 0.0, 0.0);
                let mut s = 0;
                while s < samples_per_pixel {
                    let u = ((pixel.x as f64) + random_double()) / ((width - 1) as f64);
                    let v =
                        (((height - pixel.y - 1) as f64) + random_double()) / ((height - 1) as f64);
                    let r = _cam.get_ray(u, v);
                    pixel_color +=
                        ray_color(&r, background, _world.as_ref(), _lights.as_ref(), max_depth);
                    s += 1;
                }
                color_list.push((pixel, pixel_color));
                num += 1;
                if num == width {
                    num = 0;
                    pb.inc(1);
                }
            }
            tx.send(color_list).unwrap();
            pb.finish();
        });
        threads.push(handle);
    }
    multi_progress.join_and_clear().unwrap();

    for receiver in recv.iter().take(THREAD_NUM) {
        let received = receiver.recv().unwrap();
        for pixel in received {
            color::write_color(&mut img, pixel.0, pixel.1, samples_per_pixel);
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }

    //single_thread
    // let progress = if option_env!("CI").unwrap_or_default() == "true" {
    //     ProgressBar::hidden()
    // } else {
    //     ProgressBar::new((height * width) as u64)
    // };
    // for j in 0..height {
    //     for i in 0..width {
    //         //获得(i,j)对应的（R,G,B）
    //         let pixel = img.get_pixel_mut(i, height - j - 1);
    //         let mut pixel_color = Vect3::new(0.0, 0.0, 0.0);

    //         for _s in 0..samples_per_pixel {
    //             let u: f64 = ((i as f64) + random_double()) / (width as f64);
    //             let v: f64 = ((j as f64) + random_double()) / (height as f64);
    //             let r: Ray = cam.get_ray(u, v);
    //             pixel_color += ray_color(&r, background, &world, max_depth);
    //         }
    //         let scale: f64 = 1.0 / (samples_per_pixel as f64);
    //         let r: f64 = (scale * pixel_color[0]).sqrt();
    //         let g: f64 = (scale * pixel_color[1]).sqrt();
    //         let b: f64 = (scale * pixel_color[2]).sqrt();
    //         *pixel = image::Rgb([
    //             ((256_f64) * clamp(r, 0.0, 0.999)) as u8,
    //             ((256_f64) * clamp(g, 0.0, 0.999)) as u8,
    //             ((256_f64) * clamp(b, 0.0, 0.999)) as u8,
    //         ]);
    //         progress.inc(1);
    //     }
    // }
    // progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
    exit(0);
}
