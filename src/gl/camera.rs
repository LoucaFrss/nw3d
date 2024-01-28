use core::f32::consts::PI;

use crate::glm::{vec3, Vec3};
use num_traits::Float;
#[derive(Clone, Debug)]
pub struct Camera {
    pub pos: Vec3<f32>,
    pub vel: Vec3<f32>,
    pub rot: Vec3<f32>,
    pub width: f32,
    pub height: f32,
    pub znear: f32,
    pub zfar: f32,
    pub fov: f32,
}

impl Camera {
    pub fn translation(&self) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
        let pos = self.pos;
        move |v| v - pos
    }
    // ~using quaternions, don't work properly, still don't know how to add the rotations.
    // pub fn rotation(&self) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
    //     let mut q = self.rot;
    //     q.v = q.v.normalize();

    //     move |v| {
    //         let p = Quaternion::new(0., v);
    //         let mut q = q.clone();
    //         q.v *= (q.s * 0.5).sin();
    //         q.s = (q.s * 0.5).cos();

    //         (q * p * q.inverse()).v
    //     }
    // }

    // using rotation matrices
    pub fn rotation(&self) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
        // https://en.m.wikipedia.org/wiki/3D_projection
        /*
        precalculated:
            [1      0       0    ]   [cos(c.y) 0 -sin(c.y)]   [ cos(c.z) sin(c.z) 0]
        d = [0  cos(c.x) sin(c.x)] x [    0    1     0    ] x [-sin(c.z) cos(c.z) 0]
            [0 -sin(c.x) cos(c.x)]   [sin(c.y) 0  cos(c.y)]   [     0        0    1]
        */
        let c = vec3(self.rot.x.cos(), self.rot.y.cos(), self.rot.z.cos());
        let s = vec3(self.rot.x.sin(), self.rot.y.sin(), self.rot.z.sin());
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

    pub fn projection(&self) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
        let width = self.width;
        let height = self.height;

        let c = (self.fov * 0.5).tan();
        move |v| {
            let x = v.x * (height / width) / (c * v.z);

            let y = v.y / (c * v.z);

            vec3(x, y, v.z)
        }
    }

    pub fn viewport(&self) -> impl FnMut(Vec3<f32>) -> Vec3<f32> {
        let width = self.width;
        let height = self.height;
        move |v| vec3((v.x + 1.) * (width * 0.5), (v.y + 1.) * (height * 0.5), v.z)
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            vel: vec3(0.0, 0.0, 0.0),
            rot: vec3(0.0, 0.0, 0.0),
            width: 320.,
            height: 240.,
            znear: 0.,
            zfar: 1000.0,
            fov: -PI * 0.5,
        }
    }
}
