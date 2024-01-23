use core::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};
use num_traits::Float;

use super::vec3::{vec3, Vec3};
pub trait Transformation<T> {
    fn transform(&self, value: T) -> T;
}

pub trait Transform<T: Transformation<U>, U> {
    fn transform(&mut self, trans: T);
}

pub struct Rotation<T> {
    s: Vec3<T>,
    c: Vec3<T>,
}

impl<T: Float> Rotation<T> {
    pub fn new(rot: Vec3<T>) -> Self {
        Self {
            s: Vec3 {
                x: rot.x.sin(),
                y: rot.y.sin(),
                z: rot.z.sin(),
            },
            c: Vec3 {
                x: rot.x.cos(),
                y: rot.y.cos(),
                z: rot.z.cos(),
            },
        }
    }
}
use core::fmt::Debug;
impl<
        T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy + From<f32> + Debug,
    > Transformation<Vec3<T>> for Rotation<T>
{
    fn transform(&self, v: Vec3<T>) -> Vec3<T> {
        // https://en.m.wikipedia.org/wiki/3D_projection
        /*
        precalculated:
            [1      0       0    ]   [cos(c.y) 0 -sin(c.y)]   [ cos(c.z) sin(c.z) 0]
        d = [0  cos(c.x) sin(c.x)] x [    0    1     0    ] x [-sin(c.z) cos(c.z) 0]
            [0 -sin(c.x) cos(c.x)]   [sin(c.y) 0  cos(c.y)]   [     0        0    1]
        */
        let c = self.c;
        let s = self.s;
        // println!("\n\n{:?}\n{:?}", c, s);
        let x = v.x;
        let y = v.y;
        let z = v.z;
        Vec3 {
            z: (c.x * (c.y * z + s.y * (s.z * y + c.z * x)) - s.x * (c.z * y - s.z * x)),
            x: c.y * (s.z * y + c.z * x) - s.y * z,
            y: s.x * (c.y * z + s.y * (s.z * y + c.z * x)) + c.x * (c.z * y - s.z * x),
        }
    }
}

pub struct Translation<T> {
    d: Vec3<T>,
}

impl<T> Translation<T> {
    pub fn new(d: Vec3<T>) -> Self {
        Self { d }
    }
}

impl<T: Add<T, Output = T> + Copy> Transformation<Vec3<T>> for Translation<T> {
    fn transform(&self, value: Vec3<T>) -> Vec3<T> {
        self.d + value
    }
}

pub struct Perspective<T> {
    aspect_ratio: T,
    znear: T,
    zfar: T,
    fov: T,
}

impl<T: Float> Perspective<T> {
    pub fn new(aspect_ratio: T, znear: T, zfar: T, fov: T) -> Self {
        Self {
            aspect_ratio,
            znear,
            zfar,
            fov,
        }
    }
    pub fn zclip(&self, value: Vec3<T>) -> bool {
        value.z > self.znear && value.z < self.zfar
    }
}

impl Transformation<Vec3<f32>> for Perspective<f32> {
    fn transform(&self, v: Vec3<f32>) -> Vec3<f32> {
        vec3(
            (1. / ((self.fov / 2.0).tan() * v.z)) * v.x,
            (1. / ((self.fov / 2.0).tan() * v.z)) * v.y,
            1.,
        )
    }
}

// impl<'a, T: Transformation<&'a mut E>, I: Iterator<Item = &'a mut E>, E> Transform<T, &'a mut E>
//     for I
// {
//     fn transform(&mut self, trans: T) {
//         for elem in self {
//             *elem = *trans.transform(elem);
//         }
//     }
// }
