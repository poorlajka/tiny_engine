use crate::vec3::{Vec3, dot};

pub enum Shape {
    Sphere(SphereStruct),
    Polytope(PolytopeStruct)
}

impl Shape {
    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        match self {
            Shape::Sphere(SphereStruct) => 
                SphereStruct.furthest_point(direction),

            Shape::Polytope(PolytopeStruct) => 
                PolytopeStruct.furthest_point(direction)
        }
    }

    pub fn pos(&self) -> Vec3 {
        match self {
            Shape::Sphere(SphereStruct) => 
                SphereStruct.pos(),

            Shape::Polytope(PolytopeStruct) => 
                PolytopeStruct.pos(),
        }
    }

    pub fn displace(&mut self, amount: Vec3) {
        match self {
            Shape::Sphere(SphereStruct) => 
                SphereStruct.displace(amount),

            Shape::Polytope(PolytopeStruct) => 
                PolytopeStruct.displace(amount)
        }
    }
}

pub struct SphereStruct {
        pub center: Vec3,
        pub radius: f32
}

impl SphereStruct {

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        self.center + direction * self.radius
    }

    pub fn pos(&self) -> Vec3 {
        self.center 
    }

    pub fn displace(&mut self, amount: Vec3) {
        self.center += amount;
    }
}

pub struct PolytopeStruct {
        pub center: Vec3,
        pub vertices: Vec<Vec3>
}

impl PolytopeStruct {

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        let mut max_dot = 0.0;
        let mut furthest_point = Vec3::NULL_VEC;

        for vertex in &self.vertices {
            let curr_dot = dot(direction, *vertex);
            if curr_dot > max_dot {
                max_dot = curr_dot;
                furthest_point = *vertex;
            }
        }
        furthest_point
    }

    pub fn pos(&self) -> Vec3 {
        self.center 
    }

    pub fn displace(&mut self, amount: Vec3) {
        self.center += amount;
    }

    pub fn edges(&self) -> &Vec<Vec3> {
        &self.vertices
    }

}

pub fn support(shape_a: &Shape, shape_b: &Shape, direction: Vec3) -> Vec3 {
    shape_a.furthest_point(direction) - shape_b.furthest_point(-direction)
}

