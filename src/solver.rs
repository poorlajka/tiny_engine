use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use crate::phys_state::PhysState;
use bevy::prelude::Resource; 
use glam::Quat;
use crate::body::Body;

#[derive(PartialEq)]
pub enum Solver {
    Position,
    Impulse,
}

impl Solver {
    pub fn solve(&self, bodies: &mut Vec<Body>, collisions: &Vec<CData>) {
        match self {
            Solver::Position => solve_for_position(bodies, collisions),
            Solver::Impulse => solve_for_impulse(bodies, collisions),
        }
    }
}

fn solve_for_position(bodies: &mut Vec<Body>, collisions: &Vec<CData>) {
    for collision in collisions {
        let CData { id_a, id_b, normal, depth, } = *collision;

        bodies[id_a].transform.position -= normal * depth;
        bodies[id_b].transform.position += normal * depth;
    }
}

fn solve_for_impulse(bodies: &mut Vec<Body>, collisions: &Vec<CData>) {
    for collision in collisions {
        let CData { id_a, id_b, normal, depth, } = *collision;
        let (vel_a, vel_b, ang_vel_a, ang_vel_b) = impulse_response(&bodies[id_a], &bodies[id_b], normal, depth);

        bodies[id_a].vel += vel_a;
        bodies[id_b].vel -= vel_b;
        
        bodies[id_a].ang_vel += ang_vel_a/600.0;
        bodies[id_b].ang_vel -= ang_vel_b/600.0;
    }
}

fn impulse_response(obj_a: &Body, obj_b: &Body, n: Vec3, depth: f32) -> (Vec3, Vec3, Vec3, Vec3) {

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


