// a, b, c : indices of the vertices in the (TODO: primitive buffer)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub material: u8,
}

impl Face {
    pub fn new(a: u16, b: u16, c: u16, material: u8) -> Self {
        Self { a, b, c, material }
    }
}

pub fn face(a: u16, b: u16, c: u16, material: u8) -> Face {
    Face { a, b, c, material }
}
