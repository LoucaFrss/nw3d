use crate::glm::{rotation, scaling, translation, Transformation, Vec3};

use super::{Face, Material};

// still hasnt found optimal data struct to be stored in external data
// there will probably be primitives like in GLTF
// TODO: understand this: https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/figures/gltfOverview-2.0.0d-small.png
#[derive(Debug)]
pub struct Mesh<const V: usize, const F: usize, const M: usize> {
    pub vertices: [Vec3<i16>; V],
    pub faces: [Face; F],
    pub pos: Vec3<f32>,
    pub scale: Vec3<f32>,
    pub rot: Vec3<f32>,
    pub visible: [bool; V],
    pub materials: [Material; M],
}

impl<const V: usize, const F: usize, const M: usize> Mesh<V, F, M> {
    pub fn translation(&self) -> impl Transformation {
        translation(self.pos)
    }
    pub fn scaling(&self) -> impl Transformation {
        scaling(self.scale)
    }
    pub fn rotation(&self) -> impl Transformation {
        rotation(self.rot)
    }
    pub fn visible(&self, face: Face) -> bool {
        self.visible[face.a as usize]
            || self.visible[face.b as usize]
            || self.visible[face.c as usize]
    }
}

// impl Mesh {
//     pub fn new(pos: Vec3<f32>, scale: Vec3<f32>, rot: Vec3<f32>) -> Self {
//         Self { pos, scale, rot }
//     }
// }
