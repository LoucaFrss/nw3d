use crate::glm::Vec3;

// for now materials are only colors, pbr could be great but probably expensive cause of float ops
#[derive(Debug)]
pub struct Material {
    pub color: Vec3<f32>,
}

impl Material {
    pub fn new(color: Vec3<f32>) -> Self {
        Self { color }
    }
}

// should be moved somewhere
// like bufferviews in gltf
pub struct View {
    pub material: Material,
    pub offset: usize,
    pub len: usize,
}

impl View {
    pub fn new(material: Material, offset: usize, len: usize) -> Self {
        Self {
            material,
            offset,
            len,
        }
    }
}
