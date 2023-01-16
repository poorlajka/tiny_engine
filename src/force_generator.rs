use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::body::Body;


pub struct ForceGenerator {
    lin_force: Vec3,
    ang_force: Vec3,
    bodies: Vec<usize>,
}
/*
impl ForceGenerator {
    fn apply_force(&self, bodies: &mut Vec<Body>) {
        for body_id in self.bodies {
            bodies[body_id].apply_lin_force(lin_force);
            bodies[body_id].apply_ang_force(ang_force);
        }
    }
}
*/