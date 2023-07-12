use crate::*;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vect3) -> Vect3;
}

pub struct SolidColor {
    pub color_value: Vect3,
}
impl SolidColor {
    // pub fn new1(c: Vect3) -> Self {
    //     Self { color_value: (c) }
    // }
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
        Vect3::new(1.0, 1.0, 1.0) * self.noise.turb(p * self.scale, 7)
    }
}
