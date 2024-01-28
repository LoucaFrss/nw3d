use core::mem::swap;

use crate::glm::Vec3;
// Bresenham algorithm
pub fn line(
    mut a: Vec3<isize>,
    mut b: Vec3<isize>,
    framebuffer: &mut [u16; 320 * 240],
    color: u16,
) {
    let steep = (a.x - b.x).abs() < (a.y - b.y).abs();
    if steep {
        swap(&mut a.x, &mut a.y);
        swap(&mut b.x, &mut b.y);
    }
    if a.x > b.x {
        swap(&mut a.x, &mut b.x);
        swap(&mut a.y, &mut b.y);
    }

    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let de2 = dy.abs() * 2;
    let mut e2 = 0;
    let mut y = a.y;

    let mut x = a.x;
    while x <= b.x {
        // could be extracted out of the loop but seems faster like that, weird compiler optimization?
        if steep && x > 0 && x < 240 && y > 0 && y < 320 {
            framebuffer[y as usize + (x as usize) * 320] = color;
        } else if x > 0 && x < 320 && y > 0 && y < 240 {
            framebuffer[x as usize + (y as usize) * 320] = color;
        }
        e2 += de2;
        if e2 > dx {
            // same as before, could be extracted out the loop or done in branchless but seems useless, no perf difference
            y += if b.y > a.y { 1 } else { -1 };
            e2 -= dx * 2;
        }
        x += 1;
    }
}
