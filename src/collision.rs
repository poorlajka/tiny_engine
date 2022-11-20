use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape::Shape;
use crate::gjk::gjk;
use crate::epa::epa;
use crate::phys::PhysObj;
use itertools::Itertools;
use std::iter;

pub struct CData {
    pub normal: Vec3,
    pub penetration_depth: f32
}

impl Vec3 {
    pub const NO_DATA: CData = CData{ normal: Vec3::new(), penetration_depth: 0.0 };
}

fn broad_phase(possible_collisions: &mut Vec<PhysObj>, objects: &Vec<PhysObj>) {
    let possible_collisions = objects.copy().iter().tuple_combinations::<(_, _)>();
}

pub fn narrow_phase(shape_a: &Shape, shape_b: &Shape) -> (bool, CData) {
        let mut simplex: Vec<Vec3> = Vec::new();
        let intersecting = gjk(&mut simplex, shape_a, shape_b) 

        let c_data = if intersecting { epa(&simplex, shape_a, shape_b) } else { CData::NO_DATA };

        (intersecting, c_data)
    }
}
