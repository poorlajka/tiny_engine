use std::ops::{Add, AddAssign, Sub, Neg, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
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

impl Mul<[[f32; 3]; 3]> for Vec3 {
    type Output = Self;

    fn mul(self, matrix: [[f32; 3]; 3]) -> Self {
        Self {
            x: matrix[0][0]*self.x + matrix[0][1]*self.y + matrix[0][2]*self.z, 
            y: matrix[1][0]*self.x + matrix[1][1]*self.y + matrix[1][2]*self.z, 
            z: matrix[2][0]*self.x + matrix[2][1]*self.y + matrix[2][2]*self.z, 
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

	pub fn dot(self, rv: Vec3) -> f32 {
		self.x * rv.x 
		+ self.y * rv.y 
		+ self.z * rv.z
	}

	pub fn cross(self, rv: Vec3) -> Vec3 {
		Vec3 {
			x: self.y * rv.z - self.z * rv.y,
			y: self.z * rv.x - self.x * rv.z,
			z: self.x * rv.y - self.y * rv.x 
		}
	}

	pub fn perp(self, rv: Vec3) -> Vec3 {
		Self::tripple_cross(rv - self, -self, rv - self)
	}

	pub fn normalize(self) -> Vec3 {
		self * quake_rsqrt(self.dot(self))
        //self / self.len()
	}

    pub fn len(self) -> f32 {
            (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
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

    pub fn same_direction(self, rv: Vec3) -> bool {
        self.normalize().dot(rv) > 0.0
    }

	pub fn tripple_cross(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
		a.cross(b).cross(c)
	}
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


