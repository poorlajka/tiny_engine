use crate::vec3::{Vec3, cross, dot, normalize, perp};
use crate::shape::Shape;
use crate::shape;
use crate::collision;

//TODO Clean this shit up this is kinda fucked 
pub fn epa(simplex: &Vec<Vec3>, shape_a: &Shape, shape_b: &Shape) -> collision::CData {

    let mut polytope = simplex.clone();
    let mut faces = vec![
        vec![0, 1, 2], 
        vec![0, 3, 1], 
        vec![0, 2, 3], 
        vec![1, 3, 2]
    ];

    let mut min_normal = Vec3::new();
    let mut min_distance = f32::MAX;
    let (mut normals, mut min_face) = get_face_normals(&polytope, &faces);

    while min_distance == f32::MAX {
        (min_normal, min_distance) = normals[min_face];

        let support = shape::support(shape_a, shape_b, min_normal);

        let support_distance = dot(support, min_normal);

        if f32::abs(support_distance - min_distance) > 0.001 {
            min_distance = f32::MAX;

            //part 1
            let mut unique_edges: Vec<(usize, usize)> = Vec::new();
            let mut i: usize = 0;
            while i < normals.len() {
                if cross(normals[i].0, support) == Vec3::NULL_VEC {
                    add_if_unique_edge(&mut unique_edges, &faces, i, 0, 1);
                    add_if_unique_edge(&mut unique_edges, &faces, i, 1, 2);
                    add_if_unique_edge(&mut unique_edges, &faces, i, 2, 0);
                    faces[i][2] = faces[i].pop().expect("ya");
                    faces[i][1] = faces[i].pop().expect("ya");
                    faces[i][0] = faces[i].pop().expect("ya");
                    normals[i] = normals.pop().expect("ya");
                    i = i-1;
                }
            }

            //part 2
            let mut new_faces: Vec<Vec<usize>> = Vec::new();
            for (i, (edge_a, edge_b)) in unique_edges.iter().enumerate() {
               new_faces[i].push(*edge_a); 
               new_faces[i].push(*edge_b); 
               new_faces[i].push(polytope.len()); 
            }
            polytope.push(support);

            //part 3
            let (mut new_normals, mut new_min_face) = get_face_normals(&polytope, &new_faces);
            let mut old_min_distance = f32::MAX;
            for (i, (normal, distance)) in normals.iter().enumerate() {
                if *distance < old_min_distance {
                    old_min_distance = *distance;
                    min_face = i;
                }
            }
            if new_normals[new_min_face].1 < old_min_distance {
                min_face = new_min_face + normals.len();
            }
            faces.append(&mut new_faces);
            normals.append(&mut new_normals);
        }
    }

    collision::CData {
        normal: min_normal,
        penetration_depth: min_distance + 0.001
    }

}

fn get_face_normals(polytope: &Vec<Vec3>, faces: &Vec<Vec<usize>>) -> (Vec<(Vec3, f32)>, usize) {
    let mut normals: Vec<(Vec3, f32)> = Vec::new();
    let mut min_triangle = 0;
    let mut min_distance = f32::INFINITY;

    for (i, face) in faces.iter().enumerate() {
        let (a,b,c) = (polytope[face[0]], polytope[face[1]], polytope[face[2]]);

        let ab = b - a;
        let ac = c - a;

        let mut normal = normalize(cross(ab, ac));
        let mut distance = dot(normal, a);

        if distance < 0.0 {
            normal *= -1.0;
            distance *= -1.0;
        }
        normals.push((normal, distance));

        if distance < min_distance {
            min_distance = distance;
            min_triangle = i;
        }
    }
    
    (normals, min_triangle)
}

fn add_if_unique_edge(edges: &mut Vec<(usize, usize)>, faces: &Vec<Vec<usize>>, face: usize, a: usize, b: usize) {
    let reverse = edges.iter().position(|&x| x == (faces[face][b], faces[face][a]));
    match reverse {
        Some(i) => {
            edges.remove(i);
        },
        None => edges.push((faces[face][a], faces[face][b]))
    }
}



