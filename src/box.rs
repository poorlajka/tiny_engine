
pub struct box {
	pub pos: Vec3,
	pub height: f32,
	pub width: f32,
	pub bredth: f32,
	pub vertices: Vec<Vec3>,
}

impl box {

    pub fn pos(&self) -> Vec3 {
        self.trans_pos 
    }

    pub fn transform(&mut self, transform: &Transform) {
        for vertex in &mut self.trans_vertices {
            *vertex = Transform::apply(vertex, transform);
        }
        self.trans_pos = Transform::apply(pos, transform);
    }

	pub fn inv_inertia(&self, inv_m: f32) -> [[f32; 3]; 3] {
		w2 = self.width.pow(2)
		h2 = self.height.pow(2);
		d2 = self.depth.pow(2);
		
		[[12.0*inv_m/(h2+d2), 0.0, 0.0]
		 [0.0, 12.0*inv_m/(w2+h2), 0.0]
		 [0.0, 0.0, 12.0*inv_m/(w2+d2)]]
	}

    pub fn bounding_sphere(&self) -> Shape{
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
