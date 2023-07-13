// use image::GenericImageView;

use crate::*;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vect3) -> Vect3;
}

pub struct SolidColor {
    pub color_value: Vect3,
}
impl SolidColor {
    pub fn new1(c: Vect3) -> Self {
        Self { color_value: (c) }
    }
    // pub fn new2(red: f64, green: f64, blue: f64) -> Self {
    //     Self {
    //         color_value: (Vect3::new(red, green, blue)),
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vect3) -> Vect3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    // pub fn new1(_even: Rc<dyn Texture>, _odd: Rc<dyn Texture>) -> Self {
    //     Self {
    //         odd: (_odd),
    //         even: (_even),
    //     }
    // }
    // pub fn new2(c1: Vect3, c2: Vect3) -> Self {
    //     Self {
    //         odd: (Rc::new(SolidColor::new1(c2))),
    //         even: (Rc::new(SolidColor::new1(c1))),
    //     }
    // }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vect3) -> Vect3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    // pub fn new1() -> Self {
    //     Self {
    //         noise: (Perlin::new()),
    //         scale: 0.0,
    //     }
    // }
    pub fn new2(sc: f64) -> Self {
        Self {
            noise: (Perlin::new()),
            scale: (sc),
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vect3) -> Vect3 {
        Vect3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}
const COMPONENTS_PER_PIXEL: u32 = 3;
impl ImageTexture {
    // pub fn default() -> Self {
    //     Self {
    //         data: (Vec::new()),
    //         width: (0),
    //         height: (0),
    //         bytes_per_scanline: (0),
    //     }
    // }
    // pub fn new(filename: &str) -> Self {
    //     let mut ans = ImageTexture::default();
    //     let image_result = image::open(filename);

    //     // let image_result: Result<DynamicImage, _> = open(filename);
    //     match image_result {
    //         Ok(image) => {
    //             // let dimensions = image.dimensions();
    //             // ans.width = dimensions.0;
    //             // ans.height = dimensions.1;
    //             ans.width = image.width();
    //             ans.height = image.height();
    //             ans.bytes_per_scanline = COMPONENTS_PER_PIXEL * ans.width;
    //             for y in 0..ans.height {
    //                 for x in 0..ans.width {
    //                     let pixel = image.get_pixel(x, y);
    //                     ans.data.push(pixel[0]);
    //                     ans.data.push(pixel[1]);
    //                     ans.data.push(pixel[2]);
    //                 }
    //             }
    //         }
    //         Err(_err) => {}
    //     }
    //     ans
    // }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vect3) -> Vect3 {
        if self.data.is_empty() {
            return Vect3::new(0.0, 1.0, 1.0);
        }
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j > self.height {
            j = self.height - 1;
        }
        let color_scale = 1.0 / 255.0;
        let index: usize = (j * self.bytes_per_scanline + i * COMPONENTS_PER_PIXEL) as usize;
        Vect3::new(
            color_scale * self.data[index] as f64,
            color_scale * self.data[index + 1] as f64,
            color_scale * self.data[index + 2] as f64,
        )
    }
}
