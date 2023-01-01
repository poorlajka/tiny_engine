use crate::collider::Collider;
use crate::vec3::Vec3;
use crate::transform::Transform;

pub struct Cuboid {
    pub pos: Vec3,
	pub height: f32,
	pub width: f32,
	pub depth: f32,
	pub vertices: Vec<Vec3>,
    pub trans_vertices: Vec<Vec3>,
}

impl Cuboid {

    pub fn transform(&mut self, transform: &Transform) {
        for vertex in &mut self.vertices {
            *vertex = Transform::apply(*vertex, transform, self.pos);
        }
        self.pos = Transform::apply(self.pos, transform, self.pos);
    }

	pub fn inv_inertia(&self, inv_m: f32) -> [[f32; 3]; 3] {
		let w2 = self.width.powf(2.0);
		let h2 = self.height.powf(2.0);
		let d2 = self.depth.powf(2.0);
		
		[[12.0*inv_m/(h2+d2), 0.0, 0.0],
		 [0.0, 12.0*inv_m/(w2+h2), 0.0],
		 [0.0, 0.0, 12.0*inv_m/(w2+d2)]]
	}

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        let mut max_dot = f32::MIN;
        let mut furthest_point = Vec3::NULL_VEC;

        for vertex in &self.vertices {
            let curr_dot = direction.dot(*vertex - self.pos);
            if curr_dot > max_dot {
                max_dot = curr_dot;
                furthest_point = *vertex;
            }
        }
        furthest_point
    }
}
