use std::io::{self, Read};
use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Debug, Clone, Copy)]
pub struct Vect3 {
    pub e: [f64; 3],
}

impl Vect3 {
    pub fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    // fn r(&self) -> f64 {
    //     self.e[0]
    // }
    // fn g(&self) -> f64 {
    //     self.e[1]
    // }
    // fn b(&self) -> f64 {
    //     self.e[2]
    // }
    pub fn length(&self) -> f64 {
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
            result.e[i] = self.e[i] / rhs.e[i];
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

pub fn unit_vector(v: Vect3) -> Vect3 {
    v / v.length()
}
pub fn dot(v1: Vect3, v2: Vect3) -> f64 {
    let result: f64 = v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2];
    result
}