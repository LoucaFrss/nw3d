use core::f32::consts::PI;

use crate::glm::{perspective, rotation, translation, vec3, viewport, Transformation, Vec3};
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
    pub fn translation(&self) -> impl Transformation {
        translation(-self.pos)
    }
    pub fn rotation(&self) -> impl Transformation {
        rotation(self.rot)
    }
    pub fn projection(&self) -> impl Transformation {
        perspective(self.fov, self.height / self.width)
    }
    pub fn viewport(&self) -> impl Transformation {
        viewport(self.width, self.height)
    }
}

impl Camera {}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            vel: vec3(0.0, 0.0, 0.0),
            rot: vec3(0.0, 0.0, 0.0),
            width: 320.,
            height: 240.,
            znear: 5.,
            zfar: 1000.0,
            fov: -PI * 0.5,
        }
    }
}
