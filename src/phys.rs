use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use crate::phys_obj::PhysObj;
use bevy::prelude::Resource; 
use glam::Quat;


#[derive(Resource)]
pub struct PhysState {
    grav: Vec3,
    objects: Vec<PhysObj>,
}

impl PhysState {

    pub fn new() -> PhysState {
        PhysState {
            grav: Vec3 { x: 0.0, y: -0.0, z: 0.0 },
            objects: Vec::new()
        }
    }

    pub fn add_obj(&mut self, collider: Collider, mass: f32) -> usize {
        let id = self.objects.len();
        self.objects.push(PhysObj::new(collider, mass, id));

        id
    }

    pub fn remove_obj(&mut self, id: usize) {
        //TODO this will fuck up id of other objects
        self.objects.remove(id);
    }

    pub fn get_obj(&mut self, obj_id: usize) -> &mut PhysObj {
        &mut self.objects[obj_id]
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
        for collision in collisions {
            let CData { id_a, id_b, normal, depth, .. } = *collision;

            let transform = Transform::new(-normal * depth, Quat::from_rotation_z(0.0));

            self.objects[id_a].collider.transform(&transform);

            let transform = Transform::new(normal * depth, Quat::from_rotation_z(0.0)); 
            self.objects[id_b].collider.transform(&transform);
			
			let (vel_a, vel_b, ang_vel_a, ang_vel_b) = self.impulse_response(&self.objects[id_a], &self.objects[id_b], normal, depth);
			self.objects[id_a].vel += vel_a;
			self.objects[id_b].vel -= vel_b;
			
			self.objects[id_a].ang_vel += ang_vel_a/600.0;
			self.objects[id_b].ang_vel -= ang_vel_b/600.0;
        }
	}

    //TODO put in new file and fix
    fn impulse_response(&self, obj_a: &PhysObj, obj_b: &PhysObj, n: Vec3, depth: f32) -> (Vec3, Vec3, Vec3, Vec3) {

        let (v1, v2, m1, m2, i1, i2) = (
            obj_a.vel, 
            obj_b.vel, 
            obj_a.inv_mass, 
            obj_b.inv_mass, 
            obj_a.inv_inertia, 
            obj_b.inv_inertia
        );

        let (r1, r2) = (n - obj_a.collider.pos(), n - obj_b.collider.pos());

        let e = obj_a.restitution + obj_b.restitution;

        let j = (v1*(-1.0+e)).dot(n) / 
            (m1 + m2 + (r1.cross(n).cross(r1)*i1 + r2.cross(n).cross(r2)*i2).dot(n));

        let w = 50.0;

        (n*m1*j*w, n*m2*j*w, r1.cross(n*j)*i1*w, r2.cross(n*j)*i2*w)
    }
}


