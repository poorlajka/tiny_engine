
fn aabb_intersect(box_a: &Cuboid, box_b: &Cuboid) -> bool {
    if box_a.min_x > box_b.max_x {
        return false;
    }
    if box_a.max_x < box_b.min_x {
        return false
    }
    if box_a.min_y > box_b.max_y {
        return false;
    }
    if box_a.max_y < box_b.min_y {
        return false
    }
    if box_a.min_z > box_b.max_z {
        return false;
    }
    if box_a.max_z < box_b.min_z {
        return false
    }

    true
}