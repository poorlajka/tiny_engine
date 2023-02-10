use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::transform::Transform;


pub struct Body {
	pub transform: Transform,
    pub collider: Collider,

    pub inv_mass: f32,
    pub force: Vec3,
    pub vel: Vec3,

    pub inv_inertia: [[f32; 3]; 3],
    pub torque: Vec3,
    pub ang_vel: Vec3,

    pub restitution: f32,
    pub stationary: bool,
	pub ghost: bool,
    pub id: usize,
}


impl Body {
    pub fn new(collider: Collider, mass: f32, id: usize) -> Body {
        Body {
            transform: Transform::from_position(collider.pos()),

            inv_mass: 1.0/mass,
            force: Vec3::new(),
            vel: Vec3::new(), 

            inv_inertia: collider.inv_inertia(mass),
            torque: Vec3::new(),
            ang_vel: Vec3::new(),

            collider: collider,
            restitution: 0.0,
            stationary: false,
            ghost: false,
            id: id,
        }
        
    }

    pub fn apply_lin_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn apply_ang_force(&mut self, torque: Vec3) {
        self.torque += torque;
    }

    pub fn clear_forces(&mut self) {
        self.force = Vec3::NULL_VEC;
        self.torque = Vec3::NULL_VEC;
    }
 
    pub fn collider(&self) -> &Collider {
        &self.collider
    }

    pub fn set_ghost(&mut self, ghost: bool) {
        self.ghost = ghost;
    }
}
