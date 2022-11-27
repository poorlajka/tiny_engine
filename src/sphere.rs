
pub struct SphereStruct {
	pub pos: Vec3,
	pub trans_pos: Vec3,
	pub radius: f32
}

impl SphereStruct {

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

		[[2.5*inv_m/r2), 0.0, 0.0]
		 [0.0, 2.5*inv_m/r2, 0.0]
		 [0.0, 0.0, 2.5*inv_m/r2]]
	}

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        self.pos + direction * self.radius
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
