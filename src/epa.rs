use crate::vec3::Vec3;
use crate::collider::Collider;
use crate::collider::support;

pub fn epa(simplex: &Vec<Vec3>, collider_a: &Collider, collider_b: &Collider) -> (Vec3, f32) {

    //The initial polytope is the result of GJK returning true and thus contains the origin.
    //TODO polytope should probably be a struct containing normals faces and vertecies.
    let mut polytope = simplex.clone();

    let mut faces = vec![
        0, 1, 2,
        0, 3, 1,
        0, 2, 3,
        1, 3, 2
    ];

    let (mut face_normals, mut closest_face) = get_face_normals(&polytope, &faces);

    let mut closest_face_normal = Vec3::new();
    let mut closest_face_distance = f32::MAX;

    let mut iterations = 0;

    loop {
        //EPA generally does not work for two continous shapes with an infinite amount of support
        //points. However, forcing the algorithm to exit after a couple of iterations is a dirty hack that
        //kinda solves the problem.
        if iterations >= 30 {
            break;
        }

        //1. Get the face on the polytope which is closest to the origin.
        (closest_face_normal, closest_face_distance) = face_normals[closest_face];

        //2. Get the furthest point on the minkowski difference in the direction of our closest
        //   face.
        let minkowski_point = support(collider_a, collider_b, closest_face_normal);

        //3. If the closest face is part of the hull of the minkowski difference
        //   we have found the point of intersection.
        if on_minkowski_hull(closest_face_normal, closest_face_distance, minkowski_point) {
            break;
        }

        //4. Otherwise the polytope has to be expanded by adding the new point.
        else {
            closest_face = expand_polytope(
                &mut polytope, 
                &mut faces, 
                &mut face_normals, 
                minkowski_point, 
                closest_face);
        }
        iterations += 1;
    }

    (closest_face_normal, closest_face_distance)
}

fn on_minkowski_hull(face_normal: Vec3, face_distance: f32, minkowski_point: Vec3) -> bool {
    let minkowski_distance = face_normal.dot(minkowski_point);

    (minkowski_distance - face_distance).abs() < 0.003
}

//TODO Arguably kinda shit implementation should probably be rewritten from scratch.
fn expand_polytope(
    polytope: &mut Vec<Vec3>, 
    faces: &mut Vec<usize>, 
    face_normals: &mut Vec<(Vec3, f32)>, 
    minkowski_point: Vec3, 
    mut closest_face: usize) -> usize {

    let mut unique_edges: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;

    while i < face_normals.len() {
        if face_normals[i].0.same_direction(minkowski_point) {
            let f = i*3;

            add_if_unique_edge(&mut unique_edges, &faces, f, f+1);
            add_if_unique_edge(&mut unique_edges, &faces, f+1, f+2);
            add_if_unique_edge(&mut unique_edges, &faces, f+2, f);

            faces[f+2] = faces[faces.len() - 1]; faces.pop().unwrap();
            faces[f+1] = faces[faces.len() - 1]; faces.pop().unwrap();
            faces[f] = faces[faces.len() - 1]; faces.pop().unwrap();

            face_normals[i] = face_normals[face_normals.len() - 1];
            face_normals.pop().unwrap();
        }
        else {
            i += 1;
        }
    }

    let mut new_faces: Vec<usize> = Vec::new();
    for (edge_a, edge_b) in &unique_edges {
       new_faces.push(*edge_a);
       new_faces.push(*edge_b);
       new_faces.push(polytope.len());
    }
    polytope.push(minkowski_point);


    let (mut new_face_normals, mut new_closest_face) = get_face_normals(&polytope, &new_faces);
    let mut old_min_distance = f32::MAX;
    for (i, (normal, distance)) in face_normals.iter().enumerate() {
        if *distance < old_min_distance {
            old_min_distance = *distance;
            closest_face = i;
        }
    }

    if new_face_normals[new_closest_face].1 < old_min_distance {
        closest_face = new_closest_face + face_normals.len();
    }

    faces.append(&mut new_faces);
    face_normals.append(&mut new_face_normals);

    closest_face

}

fn get_face_normals(polytope: &Vec<Vec3>, faces: &Vec<usize>) -> (Vec<(Vec3, f32)>, usize) {
    let mut normals: Vec<(Vec3, f32)> = Vec::new();
    let mut min_triangle = 0;
    let mut min_distance = f32::MAX;

    let mut i: usize = 0;

    while i < faces.len() {

        let (a,b,c) = (polytope[faces[i]], polytope[faces[i+1]], polytope[faces[i+2]]);

        let ab = b - a;
        let ac = c - a;

        let mut normal = ab.cross(ac).normalize();
        let mut distance = normal.dot(a);

        if distance < 0.0 {
            normal *= -1.0;
            distance *= -1.0;
        }

        normals.push((normal, distance));

        if distance < min_distance {
            min_triangle = i/3;
            min_distance = distance;
        }
        i += 3;
    }

    (normals, min_triangle)
}

fn add_if_unique_edge(edges: &mut Vec<(usize, usize)>, faces: &Vec<usize>, a: usize, b: usize) {
    let reverse_pair = edges.iter().position(|pair_ref| *pair_ref == (faces[b], faces[a]));

    match reverse_pair {
        Some(i) => {
            edges.remove(i);
        },
        None => {
            edges.push((faces[a], faces[b]));
        }
    }
}

