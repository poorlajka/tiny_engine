use crate::vec3::Vec3;
use crate::transform::Transform;
use crate::collider::Collider;

pub struct Cone {
    pub pos: Vec3,
    pub height: f32,
    pub radius: f32,
}

impl Cone {
    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.pos = transform.position;
    }

	pub fn inv_inertia(&self, inv_m: f32) -> [[f32; 3]; 3] {
		let r2 = self.radius.powf(2.0);
		let h2 = self.height.powf(2.0);
		
		[[5.0*inv_m/(5.0*h2) + 20.0*inv_m/(3.0*r2), 0.0, 0.0],
		 [0.0, 5.0*inv_m/(5.0*h2) + 20.0*inv_m/(3.0*r2), 0.0],
		 [0.0, 0.0, 10.0*inv_m/(3.0*r2)]]
	}

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        self.pos + direction * self.radius
    }
	
}
