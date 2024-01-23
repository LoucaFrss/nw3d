use core::arch::global_asm;
use core::f32::consts::PI;

use crate::eadk::input::{Key, KeyboardState};
use crate::glm::quaternion::Quaternion;
use crate::glm::Translation;
use crate::{
    eadk::{
        display::{push_rect_uniform, wait_for_vblank},
        Rect,
    },
    glm::{
        vec3::{vec3, Vec3},
        Perspective, Rotation, Transformation,
    },
};
use num_traits::Float;

pub fn app(_external_data: &[u8]) {
    let q = Quaternion::new(0., vec3(0., 0., 0.));
    let mut fb = [0; 320 * 240];
    let vertices: [Vec3<f32>; 8] = [
        vec3(-1., -1., -1.),
        vec3(1., -1., -1.),
        vec3(-1., 1., -1.),
        vec3(1., 1., -1.),
        vec3(-1., -1., 1.),
        vec3(1., -1., 1.),
        vec3(-1., 1., 1.),
        vec3(1., 1., 1.),
    ];
    let mut cam = Camera {
        pos: vec3(-2., 0.0, 0.),
        rot: vec3(0., 0., 0.),
        aspect_ratio: 320. / 240.,
        znear: 1.,
        zfar: 1000.,
        fov: PI / 2.,
    };
    loop {
        push_rect_uniform(
            Rect {
                x: 0,
                y: 0,
                w: 320,
                h: 240,
            },
            0,
        );
        // println!("{}", x);

        let keyboard = KeyboardState::scan();
        if keyboard.key_down(Key::Down) {
            cam.rot.x -= 0.01;
        }
        if keyboard.key_down(Key::Up) {
            cam.rot.x += 0.01;
        }
        if keyboard.key_down(Key::Left) {
            cam.rot.y -= 0.01;
        }
        if keyboard.key_down(Key::Right) {
            cam.rot.y += 0.01;
        }

        if keyboard.key_down(Key::Two) {
            cam.pos.z -= 0.1;
        }
        if keyboard.key_down(Key::Eight) {
            cam.pos.z += 0.1;
        }
        if keyboard.key_down(Key::Four) {
            cam.pos.x -= 0.1;
        }
        if keyboard.key_down(Key::Six) {
            cam.pos.x += 0.1;
        }
        if keyboard.key_down(Key::Five) {
            cam.pos.y -= 0.1;
        }
        if keyboard.key_down(Key::Dot) {
            cam.pos.y += 0.1;
        }

        let rot = cam.rotation();
        let projection = cam.projection();
        let translation = cam.translation();

        vertices
            .iter()
            .map(|v| translation.transform(*v))
            .map(|v| rot.transform(v))
            .filter(|v| projection.zclip(*v))
            .map(|v| projection.transform(v))
            .filter(|v| v.x < 0. || v.x > 319. || v.y < 0. || v.x > 239.)
            .for_each(|v| {
                draw(v, &mut fb, u16::MAX);
            });

        wait_for_vblank();
    }
}
#[derive(Clone, Debug)]
pub struct Camera {
    pos: Vec3<f32>,
    rot: Vec3<f32>,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
    fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            rot: vec3(0.0, 0.0, 0.0),
            aspect_ratio: 320.0 / 240.0,
            znear: 1.0,
            zfar: 1.0,
            fov: PI / 2.0,
        }
    }
}
impl Camera {
    fn rotation(&self) -> Rotation<f32> {
        Rotation::new(self.rot)
    }
    fn translation(&self) -> Translation<f32> {
        Translation::new(-self.pos)
    }
    fn projection(&self) -> Perspective<f32> {
        Perspective::new(self.aspect_ratio, self.znear, self.zfar, self.fov)
    }

    fn projection2(&self) -> impl FnMut(Vec3<f32>) -> Option<Vec3<f32>> {
        let test = self.clone();
        move |v| {
            if v.z < test.znear || v.z > test.zfar {
                return None;
            }
            Some(vec3(
                v.x / test.aspect_ratio * ((test.fov * 0.5).tan() * v.z),
                v.x / ((test.fov * 0.5).tan() * v.z),
                (v.z - test.znear) / (test.zfar - test.znear),
            ))
        }
    }
}

fn draw(point: Vec3<f32>, _fb: &mut [u16; 320 * 240], color: u16) {
    // println!("{:?}", point);
    let x = (point.x + 160.) as u16;
    let y = (point.y + 120.) as u16;

    if x < 319 && y < 239 {
        push_rect_uniform(Rect { x, y, w: 5, h: 5 }, color);
    }
}
