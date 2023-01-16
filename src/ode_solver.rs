use crate::body::Body;
use crate::vec3::Vec3;
use glam::Quat;

//Simple numerical solver using Euler's method.
pub fn solve(body: &Body, dt: f32) -> (Vec3, Vec3, Vec3, Quat) {
    let acc = body.force * body.inv_mass;
    let vel = body.vel + acc * dt;
    let translation = vel * dt;

    let ang_acc = body.torque * body.inv_inertia;
    let ang_vel = body.ang_vel + ang_acc * dt;
    let rotation = ang_vel * dt;

    let rotation_quat = Quat::from_rotation_z(rotation.z).mul_quat(Quat::from_rotation_y(rotation.y).mul_quat(Quat::from_rotation_x(rotation.x))).normalize();

    (vel, ang_vel, translation, rotation_quat)
}





