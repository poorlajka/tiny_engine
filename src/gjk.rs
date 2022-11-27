use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape;
use crate::shape::Shape;

pub fn gjk(simplex: &mut Vec<Vec3>, shape_a: &Shape, shape_b: &Shape) -> bool { 
    simplex.push(starting_point(shape_a, shape_b));
    let mut direction = -simplex[0].normalize();
    let mut intersecting = false;

    while !intersecting {
        let new_point = shape::support(shape_a, shape_b, direction);

        if new_point.dot(direction) < 0.0 {
            break
        }
        simplex.push(new_point);
        (intersecting, direction) = handle_simplex(simplex, direction);
    }

    intersecting
}

fn starting_point(shape_a: &Shape, shape_b: &Shape) -> Vec3 {
    let direction = (shape_a.pos() - shape_b.pos()).normalize();
    shape::support(shape_a, shape_b, direction)
}


fn handle_simplex(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    match simplex.len() {
        2 => handle_line(simplex),
        3 => handle_triangle(simplex, direction),
        _ => handle_tetrahedron(simplex, direction)
    }
}

fn handle_line(simplex: &mut Vec<Vec3>) -> (bool, Vec3) {
    let a = simplex[1];
    let b = simplex[0];

    (false, a.perp(b).normalize())
}

fn handle_triangle(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3){
    let a = simplex[2];
    let b = simplex[1];
    let c = simplex[0];

    let ab_perp = perp(a, b);
    let ac_perp = perp(a, c);
    let ao = Vec3::ORIGIN - a; 

    if ab_perp.dot(ao) > 0.0 {
        simplex.remove(0);
        return (false, ab_perp.normalize());
    }
    if ac_perp.dot(ao) > 0.0 {
        simplex.remove(1);
        return (false, ac_perp.normalize());
    }

    (false, direction)
}

fn handle_tetrahedron(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3) {
    let a = simplex[3];
    let b = simplex[2];
    let c = simplex[1];
    let d = simplex[0];

    let ab = b - a; 
    let ac = c - a;
    let ad = d - a;
    let ao = Vec3::ORIGIN - a; 

    let abc = ab.cross(ac);
    let acd = ac.cross(ad);
    let adb = ad.cross(ab);

    if abc.dot(ao) > 0.0 {
        simplex.remove(0);
        return handle_triangle(simplex, direction);
    }
    if acd.dot(ao) > 0.0 {
        simplex.remove(2);
        return handle_triangle(simplex, direction);
    }
    if adb.(ao) > 0.0 {
        simplex.remove(1);
        return handle_triangle(simplex, direction);
    }

    (true, Vec3::new())
}

