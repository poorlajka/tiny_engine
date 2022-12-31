use crate::vec3::Vec3;
use glam::Quat;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub displacement: Vec3,
    pub rotation: Quat,
}


impl Transform {
    pub const ZERO: Transform = Transform { displacement: Vec3::NULL_VEC, rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) };

    pub fn new(displacement: Vec3, rotation: Quat) -> Transform {
        Transform {
            displacement: displacement,
            rotation: rotation,
        }
    }

	pub fn apply(v: Vec3, transform: &Transform, origin: Vec3) -> Vec3 {
		let Transform { displacement, rotation } = *transform;
		let rotation = rotate(v - origin, rotation) + origin;
        translate(rotation, displacement)
	}

}
pub fn translate(v: Vec3, displacement: Vec3) -> Vec3 {
    v + displacement
}

pub fn rotate(v: Vec3, rotation: Quat) -> Vec3 {

    let vec = rotation.mul_vec3(glam::Vec3{ x: v.x, y: v.y, z: v.z });
    Vec3 {
        x: vec.x,
        y: vec.y,
        z: vec.z,
    }
}


