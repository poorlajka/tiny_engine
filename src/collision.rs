use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape::Shape;
use crate::gjk::gjk;
use crate::epa::epa;
use crate::phys::PhysObj;
use itertools::Itertools;
use std::iter;

pub struct CData {
    pub normal: Vec3,
    pub penetration_depth: f32,
    //pub obj_a: PhysObj,
    //pub obj_b: PhysObj,
}

pub fn get_collisions(collisions: &mut Vec<CData>, objects: &Vec<PhysObj>) {
    let mut possible_collisions: Vec<(PhysObj, PhysObj)> = Vec::new();
    broad_phase(&mut possible_collisions, objects);
    narrow_phase(collisions, &possible_collisions);
}

fn broad_phase(possible_collisions: &mut Vec<(PhysObj, PhysObj)>, objects: &Vec<PhysObj>) {
    // Bounding sphere for every shape will be a sphere with radius equal to the furthest point
    // from 0 in any direction. For polyhedron this is the furthest vertex.
    let mut bounding_spheres = Vec::new()
    for object in &objecst {
        bounding_spheres.push(object.bounding_sphere());
    }
    let max_bounding_sphere = bounding_spheres.iter().max_by(|sphere| sphere.radius()).unwrap();

    let possible_collisions = objects.iter().tuple_combinations::<(_, _)>();
}

pub fn narrow_phase(collisions:&mut Vec<CData>, possible_collisions: &Vec<(PhysObj, PhysObj)>) {
    for (obj_a, obj_b) in possible_collisions {
        let mut simplex: Vec<Vec3> = Vec::new();
        if let intersecting = gjk(&mut simplex, &obj_a.shape(), &obj_b.shape()) { 
            collisions.push(epa(&simplex, &obj_a.shape(), &obj_b.shape()));
        }
    }
}
