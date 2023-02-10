use crate::vec3::Vec3;
use crate::gjk::gjk;
use crate::epa::epa;
use crate::body::Body;
use itertools::Itertools;
use crate::oct_tree::OctTree;

pub struct CData {
    pub normal: Vec3,
    pub depth: f32,
    pub id_a: usize,
    pub id_b: usize,
}

pub fn get_collisions(collisions: &mut Vec<CData>, bodies: &Vec<Body>) {
    narrow_phase(collisions, bodies);
}

fn broad_phase<'a>(possible_collisions: &'a mut Vec<(&'a Body, &'a Body)>, bodies: &'a Vec<Body>) {
    let mut oct_tree = OctTree::new(Vec3::NULL_VEC, 300.0, 0.21);
    for body in bodies {
        let mut temp = body.collider.bounding_box();
        OctTree::insert(&mut oct_tree, body.collider.bounding_box());
    }

    for body in bodies {
        let primitive = body.collider.bounding_box();
        let mut new_possible_collisions: Vec<usize> = vec![];
        OctTree::get_potential_collisions(&mut oct_tree, &mut new_possible_collisions, primitive); 
        for possible_collision in &new_possible_collisions {
            possible_collisions.push((&bodies[body.id], &bodies[*possible_collision]))
        }
    }
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
    }
}
