use crate::vec3::Vec3;
use glam::Quat;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub orientation: Quat,
}


impl Transform {
    pub const ZERO: Transform = Transform { position: Vec3::NULL_VEC, orientation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) };

    pub fn new(position: Vec3, orientation: Quat) -> Transform {
        Transform {
            position: position,
            orientation: orientation,
        }
    }

	pub fn apply(v: Vec3, transform: &Transform, origin: Vec3) -> Vec3 {
		let Transform { position, orientation } = *transform;

        rotate(translate(v, position) - origin, orientation) + origin
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


