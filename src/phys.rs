use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape::Shape;
use crate::collision;
use crate::shape::SphereStruct;


pub struct PhysObj {
    pub force: Vec3,
    pub vel: Vec3,
    pub shape: Shape,
    pub inverse_mass: f32,
    //pub restitution: f32,
    //pub static: bool,
}

impl PhysObj {
    pub fn new(shape: Shape, mass: f32) -> PhysObj {
        PhysObj {
            force: Vec3::new(),
            vel: Vec3::new(), 
            shape: shape,
            inverse_mass: 1.0/mass
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn clear_forces(&mut self) {
        self.force = Vec3::NULL_VEC;
    }
    
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn update(&mut self, grav: Vec3, dt: f32) {
        let acc = self.force * self.inverse_mass + grav;
        self.vel += acc * dt;
        self.shape.displace(self.vel * dt);
        self.clear_forces();
    }
}

pub struct PhysState {
    grav: Vec3,
    objects: Vec<PhysObj>
}

impl PhysState {
    pub fn new() -> PhysState {
        PhysState {
            grav: Vec3 {x: 0.0, y: -9.82, z: 0.0 },
            objects: Vec::new()
        }
    }

    pub fn add_obj(&mut self, shape: Shape, mass: f32) -> usize {
        self.objects.push(PhysObj::new(shape, mass));
        self.objects.len() - 1
    }

    pub fn get_obj(&self, obj_key: usize) -> &PhysObj {
        &self.objects[obj_key]
    }

    pub fn apply_obj_force(&mut self, obj_key: usize, force: Vec3) {
        self.objects[obj_key].apply_force(force);
    }
    
    pub fn update(&mut self, dt: f32) {
        //Handle Collisions
        let mut potential_collisions: Vec<collision::CData> = Vec::new();
        collision::broad_phase(&mut collisions, &self.objects);

        for (obj_a, obj_b) in potential_collisions {
            let (intersected, c_data) = collision::narrow_phase(&shape_a, &shape_b);
            if intersected {
                impulse_response(

            }
        }

        //Update positions
        for obj in &mut self.objects {
            obj.update(self.grav, dt);
        }

    }

}

fn impulse_response(obj1: &PhysObj, obj2: &PhysObj, c_data: &CData) {
}

