use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collision;
use crate::transform::Transform;
use crate::collision::CData;
use crate::phys_obj::PhysObj;
use crate::solver::Solver;
use bevy::prelude::Resource; 
use glam::Quat;

#[derive(Resource)]
pub struct PhysState {
    grav: Vec3,
    objects: Vec<PhysObj>,
    solvers: Vec<Solver>,
}

impl PhysState {
    pub fn new() -> PhysState {
        let solvers: Vec<Solver> = vec![
            Solver::Position, 
            Solver::Impulse,
        ];
        PhysState {
            grav: Vec3 { x: 0.0, y: -0.0, z: 0.0 },
            objects: Vec::new(),
            solvers: solvers,
        }
    }

    pub fn add_obj(&mut self, collider: Collider, mass: f32) -> usize {
        let id = self.objects.len();
        self.objects.push(PhysObj::new(collider, mass, id));

        id
    }

    pub fn remove_obj(&mut self, id: usize) {
        //TODO this will fuck up id of other objects watch that
        // one gamedev presentation don't remember it's name
        self.objects.remove(id);
    }

    pub fn get_obj(&mut self, obj_id: usize) -> &mut PhysObj {
        &mut self.objects[obj_id]
    }

    pub fn update(&mut self, dt: f32) {
        let mut collisions: Vec<CData> = Vec::new();
        collision::get_collisions(&mut collisions, &self.objects);
		self.resolve_collisions(&collisions);

        for obj in &mut self.objects {
            obj.update(self.grav, dt);
        }
        //ode_solver::solve(self, objects, dt)
    }

	fn resolve_collisions(&mut self, collisions: &Vec<CData>) {
        for (i,solver ) in self.solvers.iter().enumerate() {
            solver.solve(&mut self.objects, collisions);
        }
    }
}


