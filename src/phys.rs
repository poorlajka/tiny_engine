use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape::Shape;
use crate::collision;
use crate::shape::SphereStruct;
use crate::transform::Transform;
use crate::collision::CData;

pub struct PhysObj {
    shape: Shape,
	transform: Transform,

    inv_mass: f32,
    force: Vec3,
    vel: Vec3,

    inv_inertia: Vec3,
    torque: Vec3,
    ang_vel: Vec3,

    restitution: f32,
    stationary: bool,
	ghost: bool,
    id: usize,

}

impl PhysObj {
    pub fn new(shape: Shape, mass: f32) -> PhysObj {
        PhysObj {
            inv_mass: 1.0/mass,
            force: Vec3::new(),
            vel: Vec3::new(), 
            ang_vel: Vec3::new(),
            mo_intertia: Vec3::new(),
            shape: shape,
            restitution: 0.0,
            stationary: false,
        }
    }

    pub fn apply_lin_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn apply_ang_force(&mut self, torque: Vec3) {
        self.torque += force;
    }

    pub fn clear_forces(&mut self) {
        self.force = Vec3::NULL_VEC;
        self.torque = Vec3::NULL_VEC;
    }
 
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn update(&mut self, grav: Vec3, dt: f32) {
        let acc = self.force * self.inv_mass + grav;
        self.vel += acc * dt;
        let distance = self.vel * dt;

        let ang_acc = self.torque * self.inv_inertia;
        self.ang_vel += ang_acc * dt;
        let rotation = self.ang_vel * dt;

        self.shape.transform(self.transform);
        self.clear_forces();
    }
}

pub struct PhysState {
    grav: Vec3,
    objects: Vec<PhysObj>
}

impl PhysState {
    pub fn new() -> PhysState {
        PhysState {
            objects: Vec::new()
        }
    }

    pub fn add_obj(&mut self, shape: Shape, mass: f32) -> usize {
        self.objects.push(PhysObj::new(shape, mass));
        self.objects.len() - 1
    }

    pub fn get_obj(&self, obj_id: usize) -> &PhysObj {
        &self.objects[obj_id]
    }

    pub fn apply_obj_force(&mut self, obj_id: usize, force: Vec3) {
        self.objects[obj_id].apply_force(force);
    }
    
    pub fn update(&mut self, dt: f32) {
        let mut collisions: Vec<CData> = Vec::new();
        collision::get_collisions(&mut collisions, &self.objects);
		self.resolve_collisions(&collisions);

        for obj in &mut self.objects {
            obj.update(self.grav, dt);
        }
    }

	fn resolve_collisions(&mut self, collisions: &Vec<CData>) {
        for collision in &collisions {
            let (id_a, id_b) = (collision.obj_a.id, collision.obj_b.id);
			
			let (vel_a, vel_b, ang_vel_a, ang_vel_b) = impulse_response(collision);
			
			self.objects[id_a].vel += vel_a;
			self.objects[id_a].ang_vel += ang_vel_a;
			self.objects[id_b].vel -= vel_b;
			self.objects[id_b].ang_vel -= ang_vel_b;
        }
	}
}

fn impulse_response(cdata: &CData) -> (Vec3, Vec3, Vec3, Vec3) {
    let (obj_a, obj_b, n, depth) = collision;

	let (v1, v2, m1, m2, i1, i2) = (obj_a.vel, obj_b.vel, obj_a.inv_mass, 
		obj_b.inv_mass, obj_a.inv_inertia, obj_b.inv_inertia);

	let (r1, r2) = (n - obj_a.shape.pos(), n - obj_b.shape.pos());
    let e = obj_a.restitution + obj_b.restitution;

	
	let j = (-(1+e)*v1).dot(n) / 
		(m1 + m2 + (i1*r1.cross(n).cross(r1) + i2*r2.cross(n).cross(r2))).dot(n);

	(m1*j*n, m2*j*n, i1*r1.cross(j*n), i2*r2.cross(j*n))
}

