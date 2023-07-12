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
    pub fn noise(&self, p: Vect3) -> f64 {
        let i = (4.0 * p.x()) as i64 & 255;
        let j = (4.0 * p.y()) as i64 & 255;
        let k = (4.0 * p.z()) as i64 & 255;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
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
