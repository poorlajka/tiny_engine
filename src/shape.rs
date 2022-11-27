use crate::vec3::{Vec3, dot};
use crate::transform::Transform;

pub enum Shape {
    Sphere(SphereStruct),
    Box(BoxStruct),
    Cylinder(CylinderStruct),
    Cone(ConeStruct),
}

impl Shape {
	pub fn new_sphere(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

	pub fn new_box(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

	pub fn new_cylinder(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

	pub fn new_cone(pos: Vec3, radius: f32) -> Shape {
		Shape::Sphere(SphereStruct { pos: pos, radius: radius })
	}

    pub fn pos(&self) -> Vec3 {
        match self {
            Shape::Sphere(sphere) => sphere.pos(),
            Shape::Polytope(poly) => poly.pos(),
        }
    }

    pub fn inv_inertia(&mut self, mass: f32) -> [[f32; 3]; 3]{
        match self {
            Shape::Sphere(sphere) => sphere.inv_inertia(mass),
            Shape::Polytope(poly) => poly.inv_inertia(mass)
        }
    }

    pub fn transform(&mut self, transform: &Transform) {
        match self {
            Shape::Sphere(sphere) => sphere.translate(transform),
            Shape::Polytope(poly) => poly.translate(transform)
        }
    }

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        match self {
            Shape::Sphere(sphere) => sphere.furthest_point(direction),
            Shape::Polytope(poly) => poly.furthest_point(direction)
        }
    }
}

pub fn support(shape_a: &Shape, shape_b: &Shape, direction: Vec3) -> Vec3 {
    shape_a.furthest_point(direction) - shape_b.furthest_point(-direction)
}



