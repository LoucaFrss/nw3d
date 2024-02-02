use crate::glm::Vec3;

pub struct Light {
    pub pos: Vec3<f32>,
    pub ia: Vec3<f32>,
    pub id: Vec3<f32>,
    pub is: Vec3<f32>,
}

impl Light {
    pub fn new(pos: Vec3<f32>, ia: Vec3<f32>, id: Vec3<f32>, is: Vec3<f32>) -> Self {
        Self { pos, ia, id, is }
    }
}
