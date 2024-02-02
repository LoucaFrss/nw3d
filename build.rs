use std::fs::write;
use std::{
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    process::Command,
};

use obj::{Obj, ObjMaterial};
fn main() {
    println!("cargo:rerun-if-changed=meshes/mesh.obj");
    println!("cargo:rerun-if-changed=src/icon.png");
    // Turn icon.png into icon.nwi
    let output = Command::new(concat!(env!("AppData"), "/npm/nwlink.cmd"))
        .args(["png-nwi", "src/icon.png", "target/icon.nwi"])
        .output()
        .expect("Failure to launch process");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut obj = Obj::load("meshes/mesh.obj").unwrap();
    obj.load_mtls().unwrap();
    let mut faces = vec![];
    let mut vertices = vec![];

    let mut materials = vec![];
    let groups = obj.data.objects[0].groups.clone();
    for group in groups {
        let mut mat_index = 0;

        if let Some(ObjMaterial::Mtl(material)) = group.material {
            let kd = material.kd.unwrap_or_default();
            mat_index = materials.len();
            materials.push(Material {
                color: Vec3::new(kd[0], kd[1], kd[2]),
            });
        }

        let polygons = group.polys.clone();

        for polygon in polygons {
            faces.push(Face {
                a: polygon.0[0].0 as u16,
                b: polygon.0[1].0 as u16,
                c: polygon.0[2].0 as u16,
                material: mat_index as u8,
            });
        }
    }
    use std::cmp::Ordering::Equal;
    let vs = obj.data.position;
    let xmin = vs
        .iter()
        .min_by(|a, b| a[0].partial_cmp(&b[0]).unwrap_or(Equal))
        .unwrap()[0];
    let xmax = vs
        .iter()
        .max_by(|a, b| a[0].partial_cmp(&b[0]).unwrap_or(Equal))
        .unwrap()[0];

    let ymin = vs
        .iter()
        .min_by(|a, b| a[1].partial_cmp(&b[1]).unwrap_or(Equal))
        .unwrap()[1];
    let ymax = vs
        .iter()
        .max_by(|a, b| a[1].partial_cmp(&b[1]).unwrap_or(Equal))
        .unwrap()[1];

    let zmin = vs
        .iter()
        .min_by(|a, b| a[2].partial_cmp(&b[2]).unwrap_or(Equal))
        .unwrap()[2];
    let zmax = vs
        .iter()
        .max_by(|a, b| a[2].partial_cmp(&b[2]).unwrap_or(Equal))
        .unwrap()[2];

    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let dz = zmax - zmin;
    let cx = i16::MAX as f32 / dx;
    let cy = i16::MAX as f32 / dy;
    let cz = i16::MAX as f32 / dz;
    for vertex in vs {
        vertices.push(Vec3::new(
            ((vertex[0] - xmin - dx / 2.) * cx) as i16,
            ((vertex[1] - ymin - dy / 2.) * cy) as i16,
            ((vertex[2] - zmin - dz / 2.) * cz) as i16,
        ));
    }
    // materials.push(Material {
    //     color: Color(u16::MAX),
    // });
    let mesh = Mesh {
        visible: vec![false; vertices.len()],
        vertices,
        faces,
        pos: Vec3::new(0., 0., 0.),
        scale: Vec3::new(1. / cx, 1. / cy, 1. / cz),
        rot: Vec3::new(0., 0., 0.),
        materials,
    };

    write("target/mesh.rst", format!("{:#?}", mesh)).unwrap();
}
// const fn rgb(r: u8, g: u8, b: u8) -> Color {
//     Color(((r as u16 & 0b11111000) << 8) + ((g as u16 & 0b11111100) << 3) + (b as u16 >> 3))
// }
#[derive(Debug)]
pub struct Color(pub u16);
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3<i16>>,
    pub faces: Vec<Face>,
    pub pos: Vec3<f32>,
    pub scale: Vec3<f32>,
    pub rot: Vec3<f32>,
    pub visible: Vec<bool>,
    pub materials: Vec<Material>,
}
#[derive(Debug)]
pub struct Material {
    pub color: Vec3<f32>,
}
#[derive(Debug)]

pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub material: u8,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub y: T,
    pub x: T,
    pub z: T,
}
use std::fmt::Debug;
impl<T: Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec3({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

pub type Vertex<T> = Vec3<T>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Clone> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        Self {
            y: value[0].clone(),
            x: value[1].clone(),
            z: value[2].clone(),
        }
    }
}
impl Vec3<f32> {
    pub fn normalize(self) -> Self {
        let norm = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn dot(self, rhs: Vec3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl<T: Mul<Output = T> + Sub<Output = T> + Clone> Vec3<T> {
    pub fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z * rhs.x.clone() - self.x.clone() * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: AddAssign<Rhs>, Rhs> AddAssign<Vec3<Rhs>> for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<Rhs>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl<T: SubAssign<Rhs>, Rhs> SubAssign<Vec3<Rhs>> for Vec3<T> {
    fn sub_assign(&mut self, other: Vec3<Rhs>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Add<Rhs, Output = O>, Rhs, O> Add<Vec3<Rhs>> for Vec3<T> {
    type Output = Vec3<O>;
    fn add(self, rhs: Vec3<Rhs>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T: Sub<Rhs, Output = O>, Rhs, O> Sub<Vec3<Rhs>> for Vec3<T> {
    type Output = Vec3<O>;
    fn sub(self, rhs: Vec3<Rhs>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Mul<Rhs, Output = O>, Rhs: Clone + VecElement, O> Mul<Rhs> for Vec3<T> {
    type Output = Vec3<O>;
    fn mul(self, rhs: Rhs) -> Self::Output {
        Self::Output {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}

impl<T: MulAssign<Rhs>, Rhs: Clone> MulAssign<Rhs> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Rhs) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = T;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl From<Vec3<f32>> for Vec3<i16> {
    fn from(value: Vec3<f32>) -> Self {
        Self {
            x: value.x as i16,
            y: value.y as i16,
            z: value.z as i16,
        }
    }
}
impl<T: Neg<Output = O>, O> Neg for Vec3<T> {
    type Output = Vec3<O>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub trait VecElement {}
#[macro_export]
macro_rules! impl_vec_elem {
    ($($type:ty),*) => {
        $(

            impl VecElement for $type {}
        )*
    };
}
impl_vec_elem!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
