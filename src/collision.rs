use crate::vec3::Vec3;
use crate::gjk::gjk;
use crate::epa::epa;
use crate::body::Body;
use itertools::Itertools;
use crate::oct_tree::OctTree;
use crate::bounding_box::BoundingBox;
use std::collections::HashSet;

pub struct CData {
    pub normal: Vec3,
    pub depth: f32,
    pub id_a: usize,
    pub id_b: usize,
}

pub fn get_collisions(collisions: &mut Vec<CData>, bodies: &Vec<Body>, oct_tree_debug: &mut Vec<BoundingBox>) {
    let mut potential_collisions: Vec<(usize, usize)> = vec![];
    broad_phase(&mut potential_collisions, bodies, oct_tree_debug);
    narrow_phase(collisions, &potential_collisions, bodies);
}

fn broad_phase(possible_collisions: &mut Vec<(usize, usize)>, bodies: &Vec<Body>, oct_tree_debug: &mut Vec<BoundingBox>){
    let mut oct_tree = OctTree::new(Vec3::NULL_VEC, 20.0, 2.0);
    for body in bodies {
        let mut primitive = body.collider.bounding_box();
        primitive.body_id = body.id;
        OctTree::insert(&mut oct_tree, primitive);
    }
    //OctTree::print(&oct_tree);

    OctTree::get_subdivisions(&oct_tree, oct_tree_debug);

    for body in bodies {
        let mut primitive = body.collider.bounding_box();
        primitive.body_id = body.id;
        let mut new_possible_collisions: Vec<usize> = vec![];
        OctTree::get_potential_collisions(&mut oct_tree, &mut new_possible_collisions, primitive); 

        let mut c = HashSet::new();
        for possible_collision in &new_possible_collisions {
            c.insert(possible_collision);

        }

        for co in &c {
            possible_collisions.push((body.id, **co));
        }
    }
}

pub fn narrow_phase(collisions:&mut Vec<CData>, possible_collisions: &Vec<(usize, usize)>, bodies: &Vec<Body>) {
    /*
    let possible_collisions = bodies.iter()
        .tuple_combinations();
        */

    for (id_a, id_b) in possible_collisions {
        //println!("POTENTIAL COLLISION BETWEEN: {}, {}", id_a, id_b);

        let (obj_a, obj_b) = (&bodies[*id_a], &bodies[*id_b]);
        let (collider_a, collider_b) = (obj_a.collider(), obj_b.collider());
        //(bodies[*id_a].collider(), bodies[*id_b].collider());

        let mut simplex: Vec<Vec3> = Vec::new();

        if gjk(&mut simplex, collider_a, collider_b){ 
        //println!("enter epa");
            let (normal, depth) = epa(&simplex, collider_a, collider_b);
        //println!("exit epa");

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
