use crate::vec3::Vec3;
use crate::transform::Transform;
use crate::cuboid::Cuboid;
use crate::sphere::Sphere;
use crate::cylinder::Cylinder;
use crate::cone::Cone;

pub enum Collider {
    Cuboid(Cuboid),
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Collider {
	pub fn new_cuboid(pos: Vec3, height: f32, width: f32, depth: f32) -> Collider {
        let mut vertices: Vec<Vec3> = Vec::new();

        vertices.push(pos + Vec3{x: height/2.0, y: width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: width/2.0, z: -depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: -width/2.0, z: -depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: -depth/2.0 }); 

        let mut trans_vertices = vertices.clone();

		Collider::Cuboid(
            Cuboid { 
                pos: pos, 
                height: height, 
                width: width, 
                depth: depth, 
                vertices: vertices, 
                trans_vertices: trans_vertices,
            }
        )
	}

	pub fn new_sphere(pos: Vec3, radius: f32) -> Collider {
		Collider::Sphere(Sphere { pos: pos, radius: radius })
	}

	pub fn new_cylinder(pos: Vec3, height: f32, radius: f32) -> Collider {
		Collider::Cylinder(Cylinder { pos: pos, height: height, radius: radius })
	}

	pub fn new_cone(pos: Vec3, height: f32, radius: f32) -> Collider {
		Collider::Cone(Cone { pos: pos, height: height, radius: radius })
	}

    pub fn pos(&self) -> Vec3 {
        match self {
            Collider::Cuboid(cuboid) => cuboid.pos(),
            Collider::Sphere(sphere) => sphere.pos(),
            Collider::Cylinder(cylinder) => cylinder.pos(),
            Collider::Cone(cone) => cone.pos(),
        }
    }
    
    pub fn inv_inertia(&self, mass: f32) -> [[f32; 3]; 3]{
        match self {
            Collider::Cuboid(cuboid) => cuboid.inv_inertia(mass),
            Collider::Sphere(sphere) => sphere.inv_inertia(mass),
            Collider::Cylinder(cylinder) => cylinder.inv_inertia(mass),
            Collider::Cone(cone) => cone.inv_inertia(mass),
        }
    }

    pub fn transform(&mut self, transform: &Transform) {
        match self {
            Collider::Cuboid(cuboid) => cuboid.transform(transform),
            Collider::Sphere(sphere) => sphere.transform(transform),
            Collider::Cylinder(cylinder) => cylinder.transform(transform),
            Collider::Cone(cone) => cone.transform(transform),
        }
    }

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        match self {
            Collider::Cuboid(cuboid) => cuboid.furthest_point(direction),
            Collider::Sphere(sphere) => sphere.furthest_point(direction),
            Collider::Cylinder(cylinder) => cylinder.furthest_point(direction),
            Collider::Cone(cone) => cone.furthest_point(direction),
        }
    }
}

pub fn support(shape_a: &Collider, shape_b: &Collider, direction: Vec3) -> Vec3 {
    shape_a.furthest_point(direction) - shape_b.furthest_point(-direction)
}



