use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use bevy::prelude::Resource; 
use glam::Quat;


pub struct PhysObj {
	pub transform: Transform,
    pub collider: Collider,

    pub inv_mass: f32,
    pub force: Vec3,
    pub vel: Vec3,

    pub inv_inertia: [[f32; 3]; 3],
    pub torque: Vec3,
    pub ang_vel: Vec3,
    pub rotation: Quat,

    pub restitution: f32,
    pub stationary: bool,
	pub ghost: bool,
    pub id: usize,
}


impl PhysObj {
    pub fn new(collider: Collider, mass: f32, id: usize) -> PhysObj {
        PhysObj {
            transform: Transform::ZERO,

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
            rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
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

    pub fn update(&mut self, grav: Vec3, dt: f32) {
        let mut acc = self.force * self.inv_mass;
        /*
        if !self.ghost {
            acc += grav;
        }
        */
        self.vel += acc * dt;
        let distance = self.vel * dt;

        if self.vel != Vec3::NULL_VEC {
            self.vel -= self.vel * 0.01;
        }

        let ang_acc = self.torque * self.inv_inertia;
        self.ang_vel += ang_acc * dt;

        let rotation = self.ang_vel * dt;

        //TODO fix this shit lol
        self.rotation = Quat::from_rotation_z(rotation.z).mul_quat(Quat::from_rotation_y(rotation.y).mul_quat(Quat::from_rotation_x(rotation.x))).normalize();

        if self.ang_vel != Vec3::NULL_VEC {
            self.ang_vel -= self.ang_vel * 0.01;
        }

        let transform = Transform::new(distance, self.rotation); 

        self.collider.transform(&transform);
        self.clear_forces();
    }
}
