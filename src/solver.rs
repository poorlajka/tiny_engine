use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use crate::phys::PhysState;
use bevy::prelude::Resource; 
use glam::Quat;
use crate::phys_obj::PhysObj;

pub enum Solver {
    Position,
    Impulse,
}

impl Solver {
    pub fn solve(&self, objects: &mut Vec<PhysObj>, collisions: &Vec<CData>) {
        match self {
            Solver::Position => solve_for_position(objects, collisions),
            Solver::Impulse => solve_for_impulse(objects, collisions),
        }
    }
}

fn solve_for_position(objects: &mut Vec<PhysObj>, collisions: &Vec<CData>) {
    for collision in collisions {
        let CData { id_a, id_b, normal, depth, } = *collision;

        let transform = Transform::new(-normal * depth, Quat::from_rotation_z(0.0));

        objects[id_a].collider.transform(&transform);

        let transform = Transform::new(normal * depth, Quat::from_rotation_z(0.0)); 
        objects[id_b].collider.transform(&transform);
    }
}

fn solve_for_impulse(objects: &mut Vec<PhysObj>, collisions: &Vec<CData>) {
    for collision in collisions {
        let CData { id_a, id_b, normal, depth, } = *collision;
        let (vel_a, vel_b, ang_vel_a, ang_vel_b) = impulse_response(&objects[id_a], &objects[id_b], normal, depth);

        objects[id_a].vel += vel_a;
        objects[id_b].vel -= vel_b;
        
        objects[id_a].ang_vel += ang_vel_a/600.0;
        objects[id_b].ang_vel -= ang_vel_b/600.0;
    }
}

fn impulse_response(obj_a: &PhysObj, obj_b: &PhysObj, n: Vec3, depth: f32) -> (Vec3, Vec3, Vec3, Vec3) {

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


