use core::{
    convert::TryInto,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::real::Real;
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Vec3<T> {
    pub y: T,
    pub x: T,
    pub z: T,
}

pub type Vertex<T> = Vec3<T>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
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

    pub fn from_be_bytes(data: &[u8; 12]) -> Self {
        Self {
            y: f32::from_be_bytes(data[0..4].try_into().unwrap()),
            x: f32::from_be_bytes(data[4..8].try_into().unwrap()),
            z: f32::from_be_bytes(data[8..12].try_into().unwrap()),
        }
    }
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
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
