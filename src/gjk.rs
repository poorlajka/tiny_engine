use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collider;

pub fn gjk(simplex: &mut Vec<Vec3>, collider_a: &Collider, collider_b: &Collider) -> bool {

    //1. Pick a starting point and direction for the simplex.
    simplex.push(pick_starting_point(collider_a, collider_b));
    let mut direction = -simplex[0].normalize();

    let mut origin_in_simplex = false;

    // If the simplex contains the origin that means that the minkowski difference of the two shapes
    // must also contain the origin and thus they are intersecting.
    let mut iterations = 0;
    while !origin_in_simplex {
        if iterations >= 100 {
            break;
        }
        iterations += 1;

        //2. Create a new point to be added to the simplex.
        let new_point = collider::support(collider_a, collider_b, direction);

        //3. Evaluate weather it is infeasable that the origin will ever be contained in the
        //   simplex.
        if new_point.dot(direction) <= 0.0 {
            break;
        }
        simplex.push(new_point);

        //3. Evaluate weather the simplex contains the origin and update the search direction.
        (origin_in_simplex, direction) = check_simplex(simplex, direction);
    }

    origin_in_simplex
}

fn pick_starting_point(collider_a: &Collider, collider_b: &Collider) -> Vec3 {
    let direction = (collider_a.pos() - collider_b.pos()).normalize();
    collider::support(collider_a, collider_b, direction)
}


fn check_simplex(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    match simplex.len() {
        2 => check_line(simplex, direction),
        3 => check_triangle(simplex, direction),
        _ => check_tetrahedron(simplex, direction)
    }
}

fn check_line(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    let a = simplex[1];
    let b = simplex[0];

    let ab = b - a;
    let ao = -a;

    if ab.same_direction(ao) {
        return (false, a.perp(b).normalize());
    }
    else {
        simplex.remove(0);
        return (false, ao.normalize());
    }
}

fn check_triangle(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    let a = simplex[2];
    let b = simplex[1];
    let c = simplex[0];

    let ab = b - a;
    let ac = c - a;
    let ao = -a;

    let abc = ab.cross(ac);

    let ac_perp = ab.cross(abc);
    let ab_perp = abc.cross(ac);

    if ab_perp.same_direction(ao) {
        if ac.same_direction(ao) {
            simplex.remove(1);
            return (false, a.perp(c).normalize());
        }
        else {
            simplex.remove(0);
            return check_line(simplex, direction);
        }
    }

    else {
        if ac_perp.same_direction(ao) {
            simplex.remove(0);
            return check_line(simplex, direction);
        }
        else {
            if abc.same_direction(ao) {
                return (false, abc.normalize());
            }
            else {
                simplex[1] = c;
                simplex[0] = b;
                return (false, -abc.normalize());
            }
        }
    }
}

fn check_tetrahedron(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    let a = simplex[3];
    let b = simplex[2];
    let c = simplex[1];
    let d = simplex[0];

    let ab = b - a;
    let ac = c - a;
    let ad = d - a;
    let ao = -a;

    let abc = ab.cross(ac);
    let acd = ac.cross(ad);
    let adb = ad.cross(ab);

    if abc.same_direction(ao) {
        simplex.remove(0);
        return check_triangle(simplex, direction);
    }
    if acd.same_direction(ao) {
        simplex.remove(2);
        return check_triangle(simplex, direction);
    }
    if adb.same_direction(ao) {
        simplex.remove(1);
        simplex[0] = b;
        simplex[1] = d;
        return check_triangle(simplex, direction);
    }

    (true, direction)
}

