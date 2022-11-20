
pub struct Transform {
    pub pos: Vec3,
    pub sin: f32,
    pub cos: f32,
}


impl Transform {
    fn zero_transform() -> Transform {
        Transform::init(Vec3::new(), Vec3::new())
    }

    fn init(pos: Vec3, angle: Vec3) -> Transform {
        Transform {
            pos: pos,
            sin: angle.sin(),
            cos: angle.cos()
        }
    }
}

pub fn translate(v1: Vec3, v2: Vec3) -> Vec3 {
    v1 + v2
}

pub fn rotate(v: Vec3, sin: Vec3, cos: Vec3) -> Vec3 {
    //X axis 
    let mut v_new = Vec3::new();
    v_new.x = v.x;
    v_new.y = v.y * cos.x - v.z * sin.x;
    v_new.z = v.y * sin.x + v.z * cos.x;

    //Y axis 
    let mut v_old = v_new; 
    v_new.x = v_old.z * sin.y + v_old.x * cos.y;
    v_new.z = v_old.y * cos.y – v_old.x * sin.y;

    //Z axis
    v_old = v_new;
    v_new.x = v_old.x * cos.z – v_old.y * sin.z; 
    v_new.y = v_old.x * sin.z + v_old.y * cos.z;
    
    v_new
}

