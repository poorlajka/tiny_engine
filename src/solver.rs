use crate::vec3::Vec3;
use crate::shape3::Shape;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use bevy::prelude::Resource; 
use glam::Quat;
 
pub enum Solver {
    Position(PositionSolver),
    Impulse(ImpulseSolver),
}

struct PositionSolver;
struct ImpulseSolver;

impl Solver {
    fn solve(&self) {
        match self {
            Position(position_solver) => position_solver.solve(),
            Impulse(impulse_solver) => impulse_solver.solve(),
        }
    }
}

impl PositionSolver {
    fn solve(collisions: Vec<CData>) {

    }
}

impl ImpulseSolver {
    fn solve(collisions: Vec<CData>) {
    }
}

