// could be useful with quaternions

use core::ops::{Add, Div, Mul, Sub};
use num_traits::Float;
pub struct Complex {
    r: f32,
    i: f32,
}

impl Complex {}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            i: self.i - rhs.i,
        }
    }
}
impl Sub<f32> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r - rhs,
            i: self.i,
        }
    }
}

impl Add<f32> for Complex {
    type Output = Complex;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r + rhs,
            i: self.i,
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            i: self.i * rhs,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl Div<f32> for Complex {
    type Output = Complex;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            i: self.i / rhs,
        }
    }
}

impl Complex {
    pub fn conjugate(&self) -> Self {
        Self {
            r: self.r,
            i: -self.i,
        }
    }

    pub fn norm(&self) -> f32 {
        (self.r * self.r - self.i * self.i).sqrt()
    }
}
