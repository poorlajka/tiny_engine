use crate::Vec3;

pub struct Transform {
    pub displacement: Vec3,
    pub sin: Vec3,
    pub cos: Vec3,
}


impl Transform {
    pub const ZERO: Transform = Transform { displacement: Vec3::NULL_VEC, sin: Vec3::NULL_VEC, cos: Vec3::NULL_VEC };

    pub fn new(displacement: Vec3, angle: Vec3) -> Transform {
        Transform {
            displacement: displacement,
            sin: angle.sin(),
            cos: angle.cos()
        }
    }

	pub fn apply(v: Vec3, transform: Transform) -> Vec3 {
		let (displacement, sin, cos) = transform;

		translate(rotate(v, sin, cos), displacement)
	}

    pub fn translate(v: Vec3, displacement: Vec3) -> Vec3 {
        v + self.displacement
    }

    pub fn rotate(v: Vec, sin: Vec3, cos: Vec3) -> Vec3 {
        //X axis 
        let mut v_new = Vec3 {
            x: v.x,
            y: v.y * cos.x - v.z * sin.x,
            z: v.y * sin.x + v.z * cos.x,
        };

        //Y axis 
        v_new = Vec3 {
            x: v_new.z * sin.y + v_new.x * cos.y,
            y: v_new.y,
            z: v_new.y * cos.y - v_new.x * sin.y
        };

        //Z axis 
        Vec3 {
            x: v_new.x * cos.z - v_new.y * sin.z, 
            y: v_new.x * sin.z + v_new.y * cos.z,
            z: v_new.z
        }
    }
}


