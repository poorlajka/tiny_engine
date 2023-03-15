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

        bodies[id_a].transform.position -= normal * depth * 0.2;
        bodies[id_b].transform.position += normal * depth * 0.2;
    }
}

fn solve_for_impulse(bodies: &mut Vec<Body>, collisions: &Vec<CData>) {
    for collision in collisions {
        let CData { id_a, id_b, normal, depth, } = *collision;
        //println!("{}", normal);
        let (vel_a, vel_b, ang_vel_a, ang_vel_b) = impulse_response(&bodies[id_a], &bodies[id_b], normal, depth);

        //Temporary fix for multiple collisions causing massive speedup

        if vel_a.len() < 3.0 && vel_b.len() < 3.0 {
            bodies[id_a].vel -= vel_a;
            bodies[id_b].vel += vel_b;
        }
        if ang_vel_a.len() < 0.1 {
            bodies[id_a].ang_vel += ang_vel_a/1.0;
        }
        else {
            bodies[id_a].ang_vel += ang_vel_a.normalize()/1000.0;

        }
        if ang_vel_a.len() < 0.1 {
            bodies[id_b].ang_vel -= ang_vel_b/1.0;
        }
        else {
            bodies[id_b].ang_vel -= ang_vel_b.normalize()/1000.0;

        }
        
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

    let (r1, r2) = (obj_a.collider.pos() - n, obj_b.collider.pos() - n);
    let vr = v2 - v1;
    let e = obj_a.restitution + obj_b.restitution;

    let mut j = (vr*(-1.0-e)).dot(n) / 
        (m1 + m2 + ((r1.cross(n)).cross(r1)*i1 + (r2.cross(n)).cross(r2)*i2).dot(n));

    let w = 3.5;

    return (n*m1*j*w, n*m2*j*w, r1.cross(n)*i1*j*w, r2.cross(n)*i2*j*w);
}


