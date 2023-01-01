
fn solve() {
    for body in bodies {
        body.transform.rotation += body.vel * dt;
        body.transform.rotation += body.ang_vel * dt;

        let acc = body.force * body.inv_mass;
        body.vel += acc * dt;
        let ang_acc = body.ang_vel * body.inv_inertia;
        body.ang_vel += ang_acc * dt;
    }
}
