use crate::vec3::{Vec3, dot};
use crate::transform::Transform;
use crate::shape::Shape;

pub struct ConeStruct {
    pos: Vec3,
    height: f32,
    radius: f32,
}


impl ConeStruct {
    pub fn pos(&self) -> Vec3 {
        self.trans_pos 
    }

    pub fn radius(&self) -> f32 {
        self.pos 
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.trans_pos = Transform::apply(pos, transform);
    }

    pub fn bounding_sphere(&self) -> Shape {
        self
    }

	pub fn inv_inertia(&self, inv_m: f32) -> [[f32; 3]; 3] {
		r2 = self.radius.pow(2);
		h2 = self.height.pow(2);
		
		[[5.0*inv_m/(5.0*h2) + 20.0*inv_m/(3.0*r2), 0.0, 0.0],
		 [0.0, 5.0*inv_m/(5.0*h2) + 20.0*inv_m/(3.0*r2), 0.0],
		 [0.0, 0.0, 10.0*inv_m/(3.0*r2)]]
	}

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        let mut max_dot = 0.0;
        let mut furthest_point = Vec3::NULL_VEC;

        for vertex in &self.vertices {
            let curr_dot = direction.dot(*vertex);
            if curr_dot > max_dot {
                max_dot = curr_dot;
                furthest_point = *vertex;
            }
        }
        furthest_point
    }
	
}
