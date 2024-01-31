use core::mem::swap;

use crate::{
    eadk::Color,
    glm::{vec3, Vec3},
};

use super::line;
#[inline]
pub fn triangle(
    a: Vec3<f32>,
    b: Vec3<f32>,
    c: Vec3<f32>,
    image: &mut [Color; 320 * 240],
    color: Color,
) {
    // sort vertices by y
    let mut array = [a, b, c];
    array.sort_unstable_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(core::cmp::Ordering::Equal));
    let [a, b, c] = array;
    if a.y == c.y {
        line(a.into(), c.into(), image, color);
        return;
    }
    if b.y == c.y {
        top_triangle(b, c, a, image, color);
        return;
    }
    if a.y == b.y {
        bottom_triangle(a, b, c, image, color);
        return;
    }

    // find d at same y as b for scanline algorithm
    let d = vec3((c.x - a.x) / (c.y - a.y) * (b.y - a.y) + a.x, b.y, 0.);
    top_triangle(b, d, a, image, color);
    bottom_triangle(b, d, c, image, color);
}
#[inline]

pub fn top_triangle(
    mut a: Vec3<f32>,
    mut b: Vec3<f32>,
    c: Vec3<f32>,
    image: &mut [Color; 320 * 240],
    color: Color,
) {
    if a.x > b.x {
        swap(&mut a, &mut b)
    }

    let slopeac = (c.x - a.x) / (c.y - a.y);
    let slopebc = (c.x - b.x) / (c.y - b.y);
    let mut xa = c.x;
    let mut xb = c.x;

    if c.y < 0. {
        xa -= c.y * slopeac;
        xb -= c.y * slopebc;
    }
    for y in c.y.max(0.) as i16..=b.y.min(239.) as i16 {
        // could be faster
        // let i = (y as usize) * 320;
        // image[(xa.max(0.) as usize) + i..=(xb as usize).min(319) + i].fill(color);
        for x in xa.max(0.) as i16..=xb.min(319.) as i16 {
            image[x as usize + (y as usize) * 320] = color;
        }

        xa += slopeac;
        xb += slopebc;
    }
}
#[inline]

pub fn bottom_triangle(
    mut a: Vec3<f32>,
    mut b: Vec3<f32>,
    c: Vec3<f32>,
    image: &mut [Color; 320 * 240],
    color: Color,
) {
    // globally the same as top_triangle, could be merge into one unique function but useless for now
    if a.x > b.x {
        swap(&mut a, &mut b)
    }

    let slopeac = (c.x - a.x) / (c.y - a.y);
    let slopebc = (c.x - b.x) / (c.y - b.y);
    let mut xa = a.x;
    let mut xb = b.x;
    if a.y < 0. {
        xa -= a.y * slopeac;
        xb -= a.y * slopebc;
    }

    for y in a.y.max(0.) as i16..=c.y.min(239.) as i16 {
        // let i = (y as usize) * 320;
        // image[xa as usize + i - 2..=xb as usize + i].fill(color);

        for x in xa.max(0.) as i16..=xb.min(319.) as i16 {
            image[x as usize + (y as usize) * 320] = color;
        }

        xa += slopeac;
        xb += slopebc;
    }
}
