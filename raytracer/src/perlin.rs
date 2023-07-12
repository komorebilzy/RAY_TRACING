use crate::*;
// use rand::prelude::SliceRandom;
pub struct Perlin {
    pub point_count: i64,
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<i64>,
    pub perm_y: Vec<i64>,
    pub perm_z: Vec<i64>,
}
impl Perlin {
    pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, sub) in c.iter().enumerate() {
            for (j, subsub) in sub.iter().enumerate() {
                for (k, value) in subsub.iter().enumerate() {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * value;
                }
            }
        }
        accum
    }

    // pub fn noise(&self, p: Vect3) -> f64 {
    //     let i = (4.0 * p.x()) as i64 & 255;
    //     let j = (4.0 * p.y()) as i64 & 255;
    //     let k = (4.0 * p.z()) as i64 & 255;
    //     self.ranfloat
    //         [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    // }
    pub fn noise(&self, p: Vect3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0, 0.0], [0.0, 0.0]], [[0.0, 0.0], [0.0, 0.0]]];
        for (di, sub) in c.iter_mut().enumerate() {
            for (dj, subsub) in sub.iter_mut().enumerate() {
                for (dk, value) in subsub.iter_mut().enumerate() {
                    *value = self.ranfloat[(self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize])
                        as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn permute(p: &mut [i64], n: i64) {
        for i in (n - 1)..0 {
            let target: usize = random_int(0, i) as usize;
            p.swap(i as usize, target);
        }
    }

    pub fn perlin_generate_perm() -> Vec<i64> {
        let mut p: Vec<i64> = Vec::new();
        for i in 0..256 {
            p.push(i as i64);
        }
        Perlin::permute(&mut p, 256);
        // p.shuffle(&mut rand::thread_rng());
        p
    }

    pub fn new() -> Self {
        let mut ran: Vec<f64> = Vec::new();
        for _i in 0..256 {
            ran.push(random_double());
        }
        Self {
            point_count: (256),
            ranfloat: (ran),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}
