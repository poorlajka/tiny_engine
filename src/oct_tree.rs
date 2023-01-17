use crate::vec3::Vec3;
use crate::body::Body;

pub struct OctNode {
    pub size: f32,
    pub points: [Vec3; 4],
    pub children: Vec<OctNode>,
    pub bodies: Vec<usize>,
}

pub enum Octant {
    TopLeftFront = 0,
    TopLeftBack = 1,
    TopRightFront = 2,
    TopRightBack = 3,
    BotLeftFront = 4,
    BotLeftBack = 5,
    BotRightFront = 6,
    BotRightBack = 7,
}

pub struct OctTree {
    root: OctNode,
    min_subdivision_size: f32,
}

impl OctTree {
    fn new(size: f32, min_subdivision_size: f32, points: [Vec3; 4]) -> Self {
        OctTree {
            root: OctNode {
                size: size,
                points: points,
                children: Vec::new(),
                bodies: Vec::new(),
            },
            min_subdivision_size: min_subdivision_size,
        }
    }

    fn insert(&mut self, body: &Body) {
        Self::insert_in_node(&mut self.root, body, self.min_subdivision_size);
    }

    fn insert_in_node(node: &mut OctNode, body: &Body, min_subdivision_size: f32) {

        if node.size <= min_subdivision_size {
            node.bodies.push(body.id);
            return;
        }

        let point = body.transform.position;
        let octant_index = node.get_octant(point) as usize;

        let mut child = &mut node.children[octant_index];

        Self::insert_in_node(&mut child, body, min_subdivision_size);
    }

}

impl OctNode {
    fn get_octant(&self, point: Vec3) -> Octant {
        let Vec3{x, y, z} = point;

        let node_blf = self.points[0]; // bot left front
        let node_tlf = self.points[1]; // top left front
        let node_brf = self.points[2]; // bot right front
        let node_brb = self.points[3]; // bot right back

        let mid_x = (node_blf.x + node_tlf.x) / 2.0;
        let mid_y = (node_tlf.y + node_brf.y) / 2.0;
        let mid_z = (node_tlf.z + node_brb.z) / 2.0;

        if  x <= mid_x {
            if y <= mid_y {
                if z < mid_z {
                    Octant::TopLeftFront
                }
                else {
                    Octant::TopLeftBack
                }
            }
            else {
                if z <= mid_z {
                    Octant::BotLeftFront
                }
                else {
                    Octant::BotLeftBack
                }
            }
        }
        else {
            if y <= mid_y {
                if z <= mid_z {
                    Octant::TopRightFront
                }
                else {
                    Octant::TopRightBack
                }
            }
            else {
                if z < mid_z {
                    Octant::BotRightFront
                }
                else {
                    Octant::BotRightBack
                }
            }
        }
    }
}




