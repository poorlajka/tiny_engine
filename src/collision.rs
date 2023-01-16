use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::gjk::gjk;
use crate::epa::epa;
use crate::body::Body;
use itertools::Itertools;
use std::iter;

pub struct CData {
    pub normal: Vec3,
    pub depth: f32,
    pub id_a: usize,
    pub id_b: usize,
}

pub fn get_collisions(collisions: &mut Vec<CData>, bodies: &Vec<Body>) {
    narrow_phase(collisions, bodies);
}

fn broad_phase(possible_collisions: &mut Vec<(Body, Body)>, bodies: &Vec<Body>) {
    //TODO research broad phase
}

pub fn narrow_phase(collisions:&mut Vec<CData>, bodies: &Vec<Body>) {
    let possible_collisions = bodies.iter()
        .tuple_combinations();

    for (obj_a, obj_b) in possible_collisions {

        let (collider_a, collider_b) = (obj_a.collider(), obj_b.collider());

        let mut simplex: Vec<Vec3> = Vec::new();

        if gjk(&mut simplex, collider_a, collider_b){ 

            let (normal, depth) = epa(&simplex, collider_a, collider_b);

            //println!("collision");
            collisions.push( 
                CData {
                    normal: normal,
                    depth: depth,
                    id_a: obj_a.id,
                    id_b: obj_b.id,
                }
            );
        }
        else {
            ;
            //println!("No collision")
        }
    }
}
