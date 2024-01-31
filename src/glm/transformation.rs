use super::{vec3, Vec3};
use num_traits::Float;
pub trait Transformation: FnMut(Vec3<f32>) -> Vec3<f32> {}
impl<C: FnMut(Vec3<f32>) -> Vec3<f32>> Transformation for C {}
// type Transformation = impl FnMut(Vec3<f32>) -> Vec3<f32>;

pub fn rotation(r: Vec3<f32>) -> impl Transformation {
    // https://en.m.wikipedia.org/wiki/3D_projection
    /*
    precalculated:
        [1      0       0    ]   [cos(c.y) 0 -sin(c.y)]   [ cos(c.z) sin(c.z) 0]
    d = [0  cos(c.x) sin(c.x)] x [    0    1     0    ] x [-sin(c.z) cos(c.z) 0]
        [0 -sin(c.x) cos(c.x)]   [sin(c.y) 0  cos(c.y)]   [     0        0    1]
    */
    let c = vec3(r.x.cos(), r.y.cos(), r.z.cos());
    let s = vec3(r.x.sin(), r.y.sin(), r.z.sin());
    #[inline]
    move |v| {
        let x = v.x;
        let y = v.y;
        let z = v.z;
        vec3(
            c.y * (s.z * y + c.z * x) - s.y * z,
            s.x * (c.y * z + s.y * (s.z * y + c.z * x)) + c.x * (c.z * y - s.z * x),
            c.x * (c.y * z + s.y * (s.z * y + c.z * x)) - s.x * (c.z * y - s.z * x),
        )
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Translation {
//     v: Vec3<f32>,
// }

// impl Translation {
//     pub fn new(v: Vec3<f32>) -> Self {
//         Self { v }
//     }
// }
pub fn translation(p: Vec3<f32>) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
    #[inline]
    move |v| v + p
}
// #[derive(Debug, Clone, PartialEq)]

// pub struct Perspective {
//     pub fov: f32,
//     pub aspect_ratio: f32,
// }

// impl Perspective {
//     pub fn new(fov: f32, aspect_ratio: f32) -> Self {
//         Self { fov, aspect_ratio }
//     }
// }

pub fn perspective(fov: f32, aspect_ratio: f32) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
    let c = (fov * 0.5).tan();
    #[inline]
    move |v| {
        let x = v.x * aspect_ratio / (c * v.z);

        let y = v.y / (c * v.z);

        vec3(x, y, v.z)
    }
}

// #[derive(Debug, Clone, PartialEq)]

// pub struct ViewPort {
//     pub width: f32,
//     pub height: f32,
// }

// impl ViewPort {
//     pub fn new(width: f32, height: f32) -> Self {
//         Self { width, height }
//     }
// }

pub fn viewport(width: f32, height: f32) -> impl Transformation {
    #[inline]
    move |v| vec3((v.x + 1.) * (width * 0.5), (v.y + 1.) * (height * 0.5), v.z)
}

// pub struct Scaling {
//     v: Vec3<f32>,
// }

// impl Scaling {
//     pub fn new(v: Vec3<f32>) -> Self {
//         Self { v }
//     }
// }
pub fn scaling(s: Vec3<f32>) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
    move |v| vec3(v.x * s.x, v.y * s.y, v.z * s.z)
}
