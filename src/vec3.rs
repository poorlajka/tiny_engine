use std::ops::{Add, AddAssign, Sub, Neg, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;
use crate::shape::Shape;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} \ny: {} \nz: {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar, 
            y: self.y * scalar, 
            z: self.z * scalar 
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar, 
            y: self.y / scalar, 
            z: self.z / scalar 
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * (-1.0)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar; 
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar; 
    }
}

impl Vec3 {

    pub const NULL_VEC: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ORIGIN: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn sin(self) -> Vec3 {
        Vec3 {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin()
        }

    }

    pub fn cos(self) -> Vec3 {
        Vec3 {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos()
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x 
    + a.y * b.y 
    + a.z * b.z
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x 
    }
}

pub fn tripple_cross(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    cross(cross(a, b), c)
}

pub fn perp(a: Vec3, b: Vec3) -> Vec3 {
    tripple_cross(b - a, -a, b - a)
}

pub fn normalize(a: Vec3) -> Vec3 {
    a * quake_rsqrt(dot(a, a))
}


pub fn transform(v: Vec3, transform: Transform) -> Vec3 {
    Transform::translate(Transform::rotate(v, transform.sin, transform.cos), transform.pos)
}

fn quake_rsqrt(number: f32) -> f32 {
    let mut y: f32 = number;
    unsafe {
        let mut i: i32 = std::mem::transmute::<f32, i32>(y);
        i = 0x5F375A86 - (i >> 1);
        y = std::mem::transmute::<i32, f32>(i);
    } 
    y * (1.5 - (number * 0.5 * y * y)) 
}


