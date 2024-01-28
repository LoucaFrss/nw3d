use crate::{
    eadk::{
        display::{push_rect, wait_for_vblank},
        input::{Key, KeyboardState},
        timing::millis,
        SCREEN,
    },
    gl::{camera::Camera, triangle, Face, Material},
    glm::Vec3,
    println,
};

use num_traits::Float;

const V: usize = include!("vertices.rst").len();

pub fn app(_external_data: &[u8]) {
    let mut fb = [0; 320 * 240];

    // import mesh data from files generated with build script
    // TODO: move build script from "meshes" to "nw3d"
    let mut vertices = include!("vertices.rst");
    for v in vertices.iter_mut() {
        *v = *v * 100f32;
        core::mem::swap(&mut v.x, &mut v.y);
    }
    let mut faces = include!("faces.rst");
    let white = Material::new(u16::MAX);
    // magic color, should be improve: rgb888 -> rgb...
    let orange = Material::new(51520);

    let black = Material::new(0);
    let materials = [white, orange, black];

    let mut visible: [bool; V];

    // let views = [
    //     View::new(white, 0, 312),
    //     View::new(orange, 312, 156),
    //     View::new(black, 468, 4),
    // ];

    let mut camera = Camera::default();
    let move_speed = 0.5;

    loop {
        let start = millis();
        // clear the screen 0 -> black in RGB565
        //TODO: Color struct
        fb.fill(0);

        let mut trangles: i32 = 0;
        // TODO: static keyboard state?
        let keyboard = KeyboardState::scan();

        if keyboard.key_down(Key::Down) {
            camera.rot.x += 0.03;
        }
        if keyboard.key_down(Key::Up) {
            camera.rot.x -= 0.03;
        }
        if keyboard.key_down(Key::Left) {
            camera.rot.y += 0.03;
        }
        if keyboard.key_down(Key::Right) {
            camera.rot.y -= 0.03;
        }

        if keyboard.key_down(Key::Two) {
            camera.vel.z -= move_speed * camera.rot.y.cos();
            camera.vel.x -= move_speed * camera.rot.y.sin();
        }
        if keyboard.key_down(Key::Eight) {
            camera.vel.z += move_speed * camera.rot.y.cos();
            camera.vel.x += move_speed * camera.rot.y.sin();
        }
        if keyboard.key_down(Key::Four) {
            camera.vel.z -= move_speed * camera.rot.y.sin();
            camera.vel.x += move_speed * camera.rot.y.cos();
        }
        if keyboard.key_down(Key::Six) {
            camera.vel.z += move_speed * camera.rot.y.sin();
            camera.vel.x -= move_speed * camera.rot.y.cos();
        }
        if keyboard.key_down(Key::Five) {
            camera.vel.y += move_speed;
        }
        if keyboard.key_down(Key::Dot) {
            camera.vel.y -= move_speed;
        }

        if keyboard.key_down(Key::Plus) {
            camera.fov += 0.01
        }
        if keyboard.key_down(Key::Minus) {
            camera.fov -= 0.01
        }
        camera.pos += camera.vel;

        camera.vel *= 0.93;

        let vs = vertices
            .map(camera.translation())
            .map(camera.rotation())
            // projection and viewport kinda useless cause no transformations applied in between but looks clean and is probably optimized anyway
            .map(camera.projection())
            .map(camera.viewport());

        // TODO: make it cleaner, find a better way to do it, hardcoded length is pretty annoying
        visible = core::array::from_fn(|i| {
            let v = vs[i];
            v.z > camera.znear
                && v.z < camera.zfar
                && v.x > 0.
                && v.x < 319.
                && v.y > 0.
                && v.y < 239.
        });
        // TODO: make it cleaner
        // TODO: improve with bubble sort, for now bubble sort is slower but could be optimized
        faces.sort_unstable_by(|b, a| {
            vs[a.a as usize]
                .z
                .min(vs[a.b as usize].z)
                .min(vs[a.c as usize].z)
                .partial_cmp(
                    &vs[b.a as usize]
                        .z
                        .min(vs[b.b as usize].z)
                        .min(vs[b.c as usize].z),
                )
                .unwrap_or(core::cmp::Ordering::Equal)
        });

        for face in faces {
            // clipping
            if !(visible[face.a as usize] || visible[face.b as usize] || visible[face.c as usize]) {
                continue;
            }
            let (a, b, c) = (
                vs[face.a as usize],
                vs[face.b as usize],
                vs[face.c as usize],
            );

            let ab = b - a;
            let ac = c - a;

            // backface culling
            if ab.x * ac.y - ac.x * ab.y < 0. {
                continue;
            }
            let n = ab.cross(ac).normalize();

            let l = n.dot((a).normalize());
            let [cr, cg, cb] =
                rgb565::Rgb565::from_rgb565(materials[face.material as usize - 1].color)
                    .to_rgb888_components();

            let color = rgb565::Rgb565::from_rgb888_components(
                ((cr as f32) * l) as u8,
                ((cg as f32) * l) as u8,
                ((cb as f32) * l) as u8,
            )
            .to_rgb565();
            trangles += 1;
            triangle(a, b, c, &mut fb, color);
        }

        // TODO: fix tearing if enough perf
        let end = millis();
        println!("\ntriangles: {}/{}\0", trangles, faces.len());

        // does not count the time to push the frame to the screen
        println!("FPS: {}\0", 1000. / (end - start) as f32);
        wait_for_vblank();
        push_rect(SCREEN, &fb);
    }
}

// fn bubble_sort_by<T, F: FnMut(&T, &T) -> Ordering>(arr: &mut [T], mut compare: F) {
//     let len = arr.len();
//     let mut swapped;

//     loop {
//         swapped = false;

//         for i in 0..len - 1 {
//             if let Ordering::Greater = compare(&arr[i], &arr[i + 1]) {
//                 arr.swap(i, i + 1);
//                 swapped = true;
//             }
//         }
//         if !swapped {
//             break;
//         }
//     }
// }
