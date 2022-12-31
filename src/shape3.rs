use crate::vec3::Vec3;
use crate::transform::Transform;
use crate::cuboid::CuboidStruct;
use crate::sphere::SphereStruct;

pub enum Shape {
    Cuboid(CuboidStruct),
    Sphere(SphereStruct),
    /*
    Cylinder(CylinderStruct),
    Cone(ConeStruct),
    */
}

impl Shape {
	pub fn new_cuboid(pos: Vec3, height: f32, width: f32, depth: f32) -> Shape {
        let mut vertices: Vec<Vec3> = Vec::new();
        vertices.push(pos + Vec3{x: height/2.0, y: width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: width/2.0, z: -depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: height/2.0, y: -width/2.0, z: -depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: depth/2.0 }); 
        vertices.push(pos + Vec3{x: -height/2.0, y: -width/2.0, z: -depth/2.0 }); 
		Shape::Cuboid(CuboidStruct { pos: pos, height: height, width: width, depth: depth, vertices: vertices })
	}

	pub fn new_sphere(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

    /*



	pub fn new_cylinder(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

	pub fn new_cone(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}
    */

    pub fn pos(&self) -> Vec3 {
        match self {
            Shape::Cuboid(cuboid) => cuboid.pos(),
            Shape::Sphere(sphere) => sphere.pos(),
        }
    }
    
    pub fn inv_inertia(&self, mass: f32) -> [[f32; 3]; 3]{
        match self {
            Shape::Cuboid(cuboid) => cuboid.inv_inertia(mass),
            Shape::Sphere(sphere) => sphere.inv_inertia(mass),
        }
    }

    pub fn transform(&mut self, transform: &Transform) {
        match self {
            Shape::Cuboid(cuboid) => cuboid.transform(transform),
            Shape::Sphere(sphere) => sphere.transform(transform),
        }
    }

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        match self {
            Shape::Cuboid(cuboid) => cuboid.furthest_point(direction),
            Shape::Sphere(sphere) => sphere.furthest_point(direction),
        }
    }
}

pub fn support(shape_a: &Shape, shape_b: &Shape, direction: Vec3) -> Vec3 {
    shape_a.furthest_point(direction) - shape_b.furthest_point(-direction)
}



