use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape;
use crate::shape::Shape;

pub fn gjk(simplex: &mut Vec<Vec3>, shape_a: &Shape, shape_b: &Shape) -> bool { 
    simplex.push(starting_point(shape_a, shape_b));
    let mut direction = normalize(-simplex[0]);
    let mut intersecting = false;

    while !intersecting {
        let new_point = shape::support(shape_a, shape_b, direction);

        if dot(new_point, direction) < 0.0 {
            break
        }
        simplex.push(new_point);
        (intersecting, direction) = handle_simplex(&mut simplex, direction);
    }

    intersecting
}

fn starting_point(shape_a: &Shape, shape_b: &Shape) -> Vec3 {
    let direction = normalize(shape_a.pos() - shape_b.pos());
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

    (false, normalize(perp(a, b)))
}

fn handle_triangle(simplex: &mut Vec<Vec3>, direction: Vec3) -> (bool, Vec3){
    let a = simplex[2];
    let b = simplex[1];
    let c = simplex[0];

    let ab_perp = perp(a, b);
    let ac_perp = perp(a, c);
    let ao = Vec3::ORIGIN - a; 

    if dot(ab_perp, ao) > 0.0 {
        simplex.remove(0);
        return (false, normalize(ab_perp));
    }
    if dot(ac_perp, ao) > 0.0 {
        simplex.remove(1);
        return (false, normalize(ac_perp));
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

    let abc = cross(ab, ac);
    let acd = cross(ac, ad);
    let adb = cross(ad, ab);

    if dot(abc, ao) > 0.0 {
        simplex.remove(0);
        return handle_triangle(simplex, direction);
    }
    if dot(acd, ao) > 0.0 {
        simplex.remove(2);
        return handle_triangle(simplex, direction);
    }
    if dot(adb, ao) > 0.0 {
        simplex.remove(1);
        return handle_triangle(simplex, direction);
    }

    (true, Vec3::new())
}

