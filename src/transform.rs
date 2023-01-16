use crate::vec3::Vec3;
use glam::Quat;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub orientation: Quat,
}


impl Transform {
    pub const ZERO: Transform = Transform { position: Vec3::NULL_VEC, orientation: Quat::IDENTITY };

    pub fn new(position: Vec3, orientation: Quat) -> Transform {
        Transform {
            position: position,
            orientation: orientation,
        }
    }

    pub fn from_position(position: Vec3) -> Transform {
        Transform {
            position: position,
            orientation: Quat::IDENTITY,
        }
    }

	pub fn apply(&self, v: Vec3) -> Vec3 {
        translate(rotate(v, self.orientation), self.position)
	}

}

fn translate(v: Vec3, position: Vec3) -> Vec3 {
    v + position
}

fn rotate(v: Vec3, orientation: Quat) -> Vec3 {

    let vec = orientation.mul_vec3(glam::Vec3{ x: v.x, y: v.y, z: v.z });
    Vec3 {
        x: vec.x,
        y: vec.y,
        z: vec.z,
    }
}


