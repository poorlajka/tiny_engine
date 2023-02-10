use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::collision::CData;
use crate::body::Body;
use crate::solver::Solver;
use crate::force_generator::ForceGenerator;
use bevy::prelude::Resource; 
use crate::ode_solver;
//use crate::oct_tree::{OctTree, OctNode, Region, Empty};

#[derive(Resource)]
pub struct PhysState {
    grav: Vec3,
    bodies: Vec<Body>,
    solvers: Vec<Solver>,
   // force_generators: Vec<ForceGenerator>,
}

impl PhysState {
    pub fn new() -> PhysState {
        let solvers: Vec<Solver> = vec![
            Solver::Position, 
            Solver::Impulse,
        ];
        PhysState {
            grav: Vec3 { x: 0.0, y: -0.0, z: 0.0 },
            bodies: Vec::new(),
            solvers: solvers,
        }
    }

    pub fn add_body(&mut self, collider: Collider, mass: f32) -> usize {
        let id = self.bodies.len();
        self.bodies.push(Body::new(collider, mass, id));

        id
    }

    pub fn remove_body(&mut self, id: usize) {
        //TODO this will fuck up id of other bodies there is a commonly used fix,
        //watch that one gamedev presentation don't remember it's name
        self.bodies.remove(id);
    }

    pub fn get_body(&mut self, obj_id: usize) -> &mut Body {
        &mut self.bodies[obj_id]
    }

    pub fn add_solver(&mut self, solver: Solver) {
        self.solvers.push(solver);
    }

    pub fn remove_solver(&mut self, solver: Solver) {
        self.solvers.retain(|x| *x != solver);
    }

    pub fn step(&mut self, dt: f32, steps: i32) {

        /*
        let mut oct_tree = OctTree::new(Vec3{x:0.0,y:0.0,z:0.0}, 5.5, 10.0);
        OctTree::insert(&mut oct_tree, &self.bodies[0]);
        */
        //1. Detect and resolve any collisions.
        let mut collisions: Vec<CData> = Vec::new();
        collision::get_collisions(&mut collisions, &self.bodies);
		self.resolve_collisions(&collisions);

        //2. Update current forces for state bodies.
        /*
        for force_generator in &force_generators {
            force_generator.apply_force(&mut bodies);
        }
        */

        //3. Update the position of all state bodies.
        self.solve_state(dt);
    }

	fn resolve_collisions(&mut self, collisions: &Vec<CData>) {
        for solver in self.solvers.iter() {
            solver.solve(&mut self.bodies, collisions);
        }
    }

    fn solve_state(&mut self, dt: f32){
        let drag = 0.01;
        for body in &mut self.bodies {
            /*
            if body.vel != Vec3::NULL_VEC {
                body.vel -= body.vel * drag;
            }

            */
            if body.ang_vel != Vec3::NULL_VEC {
                body.ang_vel -= body.ang_vel * drag;
            }

            let (vel, ang_vel, translation, rotation) = ode_solver::solve(&body, dt);
            body.vel = vel;
            body.ang_vel = ang_vel;

            body.transform.position += translation;
            body.transform.orientation *= rotation;
            body.collider.transform(&body.transform);

            body.clear_forces();
        }
    }
}


