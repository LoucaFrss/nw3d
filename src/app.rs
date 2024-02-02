use crate::{
    eadk::{
        display::{push_rect, wait_for_vblank},
        input::{Key, KeyboardState},
        rgb,
        timing::millis,
        Color, SCREEN,
    },
    gl::{camera::Camera, triangle, Face, Material, Mesh},
    glm::{vec3, Vec3},
    println,
};

use num_traits::Float;

const BACKGROUND_COLOR: Color = Color(0x2124);

macro_rules! keyboard {
    {$keyboard: expr,$($($key:ident), * => $action: expr),*} => {
        $(
            if $($keyboard.key_down($crate::eadk::input::Key::$key))&&* {
                $action
            }
        )*
    };
}

pub fn app(_external_data: &[u8]) {
    let mut fb = [Color(0); 320 * 240];

    // import mesh data from files generated with build script

    let mut mesh = include!("../target/mesh.rst");
    mesh.scale *= 10.;

    let mut camera = Camera::default();
    let move_speed = 0.5;

    loop {
        let start = millis();
        // clear the screen 0 -> black in RGB565
        fb.fill(BACKGROUND_COLOR);

        let mut triangles: i32 = 0;
        // TODO: static keyboard state?

        let keyboard = KeyboardState::scan();
        if keyboard.key_down(Key::Shift) {
            keyboard! {
            keyboard,
                 Down   => mesh.rot.x += 0.03,
                 Up     => mesh.rot.x -= 0.03,
                 Left   => mesh.rot.y += 0.03,
                 Right  => mesh.rot.y -= 0.03,

                 Four   => mesh.pos.x += 0.5,
                 Six    => mesh.pos.x -= 0.5,
                 Eight  => mesh.pos.z += 0.5,
                 Two    => mesh.pos.z -= 0.5,
                 Dot    => mesh.pos.y -= 0.5,
                 Five   => mesh.pos.y += 0.5,

                 Plus   => mesh.scale *= 1.01,
                 Minus  => mesh.scale *= 0.99
             }
        } else {
            keyboard! {
                keyboard,
                Down  => camera.rot.x += 0.03,
                Up    => camera.rot.x -= 0.03,
                Left  => camera.rot.y += 0.03,
                Right => camera.rot.y -= 0.03,

                Five  => camera.vel.y += move_speed,
                Dot   => camera.vel.y -= move_speed,
                Plus  => camera.fov += 0.01,
                Minus => camera.fov -= 0.01,
                Two => {
                    camera.vel.z -= move_speed * camera.rot.y.cos();
                    camera.vel.x -= move_speed * camera.rot.y.sin();
                },
                Eight =>{
                    camera.vel.z += move_speed * camera.rot.y.cos();
                    camera.vel.x += move_speed * camera.rot.y.sin();
                },
                Four => {
                    camera.vel.z -= move_speed * camera.rot.y.sin();
                    camera.vel.x += move_speed * camera.rot.y.cos();
                },
                Six => {
                    camera.vel.z += move_speed * camera.rot.y.sin();
                    camera.vel.x -= move_speed * camera.rot.y.cos();

                }
            }
        }

        camera.pos += camera.vel;

        camera.vel *= 0.93;
        let mut mesh_scaling = mesh.scaling();
        let mut mesh_rotation = mesh.rotation();
        let mut mesh_translation = mesh.translation();
        let mut camera_translation = camera.translation();
        let mut camera_rotation = camera.rotation();
        let mut camera_projection = camera.projection();
        let mut camera_viewport = camera.viewport();
        let vs = mesh.vertices.map(|v| {
            let mut v = Vec3::<f32>::from(v);
            v = mesh_scaling(v);
            v = mesh_rotation(v);
            v = mesh_translation(v);
            v = camera_translation(v);
            v = camera_rotation(v);
            v = camera_projection(v);
            // println!("{:?}                                ", v);
            // msleep(500);
            Vec3::<i16>::from(camera_viewport(v))
        });

        mesh.visible = core::array::from_fn(|i| {
            let v = vs[i];
            v.z > camera.znear as i16
                && v.z < camera.zfar as i16
                && v.x > 0
                && v.x < 319
                && v.y > 0
                && v.y < 239
        });
        // TODO: make it cleaner
        // TODO: improve with bubble sort, for now bubble sort is slower but could be optimized
        mesh.faces
            .sort_unstable_by(|b, a| vs[a.a as usize].z.cmp(&vs[b.a as usize].z));

        for face in mesh.faces {
            // clipping
            if !mesh.visible(face) {
                continue;
            }
            let (a, b, c) = (
                Vec3::<f32>::from(vs[face.a as usize]),
                Vec3::<f32>::from(vs[face.b as usize]),
                Vec3::<f32>::from(vs[face.c as usize]),
            );

            let ab = b - a;
            let ac = c - a;

            // backface culling
            if ab.x * ac.y - ac.x * ab.y > 0. {
                continue;
            }
            let n = ab.cross(ac).normalize();

            let l = n.dot((-a).normalize());
            let (cr, cg, cb) = mesh.materials[face.material as usize].color.components();

            let color = rgb(
                (cr * l * 255.) as u16,
                (cg * l * 255.) as u16,
                (cb * l * 255.) as u16,
            );
            triangles += 1;
            triangle(a, b, c, &mut fb, color);
        }

        // TODO: fix tearing if enough perf
        let end = millis();
        #[cfg(debug_assertions)]
        println!("\ntriangles: {}/{}\0", triangles, mesh.faces.len());

        // does not count the time to push the frame to the screen
        #[cfg(debug_assertions)]

        println!("FPS: {}\0", 1000. / (end - start) as f32);
        wait_for_vblank();
        push_rect(SCREEN, &fb);
        // push_rect_uniform(SCREEN, 0);
    }
}
