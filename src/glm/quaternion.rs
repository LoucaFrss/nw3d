// not used right now cause still don't know how to add rotation with quaternions
// TODO: know how to add rotation with quaternions if useful,
// rotation interpolation useless right now, maybe perf?

//https://www.haroldserrano.com/blog/developing-a-math-engine-in-c-implementing-quaternions carried this

use super::Vec3;
use core::ops::{Add, Mul, Sub};
use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub s: f32,
    pub v: Vec3<f32>,
}

impl Quaternion {
    pub fn new(s: f32, v: Vec3<f32>) -> Self {
        Self { s, v }
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Self {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
        }
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Self::Output {
        Self {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self {
            s: rhs.s * self.s - self.v.dot(rhs.v),
            v: rhs.v * self.s + self.v * rhs.s + self.v.cross(rhs.v),
        }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            s: self.s * rhs,
            v: self.v * rhs,
        }
    }
}

impl Quaternion {
    pub fn norm(&self) -> f32 {
        (self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z).sqrt()
    }
    pub fn normalize(&mut self) {
        if self.norm() == 0. {
            return;
        }
        let norm = 1. / self.norm();
        self.s *= norm;
        self.v = self.v * norm;
    }
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            s: self.s,
            v: self.v * -1.,
        }
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() * (1. / self.norm().powi(2))
    }
}
