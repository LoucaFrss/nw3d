use crate::glm::{Vec3, Vertex};

use super::{Camera, Face, Material};

// still hasnt found optimal data struct to be stored in external data
// there will probably be primitives like in GLTF
// TODO: understand this: https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/figures/gltfOverview-2.0.0d-small.png
#[derive(Debug)]
pub struct Mesh<'a, 'b, 'c> {
    pub pos: Vec3<f32>,
    pub scale: Vec3<f32>,
    pub rot: Vec3<f32>,
    pub vertices: &'a [Vertex<f32>],
    pub faces: &'b [Face],
    pub materials: &'c [Material],
}

impl<'a, 'b, 'c> Mesh<'a, 'b, 'c> {
    pub fn new(
        pos: Vec3<f32>,
        scale: Vec3<f32>,
        rot: Vec3<f32>,
        vertices: &'a [Vertex<f32>],
        faces: &'b [Face],
        materials: &'c [Material],
    ) -> Self {
        Self {
            pos,
            scale,
            rot,
            vertices,
            faces,
            materials,
        }
    }

    pub fn draw(camera: &Camera, framebuffer: &mut [u16; 320 * 240]) {}
}
