use crate::*;
// use rand::prelude::SliceRandom;
pub struct Perlin {
    pub point_count: i64,
    ranvec: Vec<Vect3>,
    pub perm_x: Vec<i64>,
    pub perm_y: Vec<i64>,
    pub perm_z: Vec<i64>,
}
impl Perlin {
    pub fn trilinear_interp(c: [[[Vect3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, sub) in c.iter().enumerate() {
            for (j, subsub) in sub.iter().enumerate() {
                for (k, value) in subsub.iter().enumerate() {
                    let weight_v = Vect3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += dot(*value, weight_v)
                        * (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww));
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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c: [[[Vect3; 2]; 2]; 2] = [
            [
                [Vect3::default(), Vect3::default()],
                [Vect3::default(), Vect3::default()],
            ],
            [
                [Vect3::default(), Vect3::default()],
                [Vect3::default(), Vect3::default()],
            ],
        ];
        for (di, sub) in c.iter_mut().enumerate() {
            for (dj, subsub) in sub.iter_mut().enumerate() {
                for (dk, value) in subsub.iter_mut().enumerate() {
                    *value = self.ranvec[(self.perm_x[((i + di as i64) & 255) as usize]
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
        let mut ran: Vec<Vect3> = Vec::new();
        for _i in 0..256 {
            ran.push(unit_vector(Vect3::random1(-1.0, 1.0)));
        }
        Self {
            point_count: (256),
            ranvec: (ran),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
    pub fn turb(&self, p: Vect3, depth: i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
