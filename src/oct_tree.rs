use crate::vec3::Vec3;
use crate::body::Body;
use crate::bounding_box::BoundingBox;
use core::slice::Iter;
use std::io;
use std::io::Write;

pub enum OctNode{
    Region(Region),
    Leaf(Leaf),
    Empty(Empty),
}

pub struct Region {
    pub bounding_box: BoundingBox,
    pub children: [Box<OctNode>; 8],
    pub primitives: Vec<BoundingBox>,
}

pub struct Leaf {
    pub bounding_box: BoundingBox,
    pub primitives: Vec<BoundingBox>,
}

pub struct Empty {
    pub bounding_box: BoundingBox,
}

pub struct OctTree {
    root: OctNode,
    min_size: f32,
}

impl OctTree {
    pub fn new(center: Vec3, size: f32, min_size: f32) -> Self {
        OctTree {
            root: OctNode::new_leaf(center, size),
            min_size: min_size,
        }
    }

    pub fn insert(oct_tree: &mut OctTree, primitive: BoundingBox) {
        OctNode::insert(&mut oct_tree.root, primitive, oct_tree.min_size);
    }

    pub fn get_potential_collisions(oct_tree: &mut OctTree, potential_collisions: &mut Vec<usize>, primitive: BoundingBox) {
        OctNode::get_potential_collisions(&mut oct_tree.root, potential_collisions, primitive);
    }

    pub fn print(oct_tree: &OctTree) {
        OctNode::print(&oct_tree.root);
        println!("");
        println!("");
        println!("");
        println!("");
    }

    pub fn get_subdivisions(oct_tree: &OctTree, subdivisions: &mut Vec<BoundingBox>) {
        OctNode::get_subdivisions(&oct_tree.root, subdivisions);
    }
}

impl OctNode {
    fn new_region(center: Vec3, size: f32) -> Self {
        let mut region = Region {
            bounding_box: BoundingBox::new(center, size),
            children: [
                //This makes me very sad when I look at it
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
            ],
            primitives: Vec::new(),
        };

        for octant in Octant::iter() {

            let subdivide_size = size / 2.0;
            let subdivide_center = octant.get_center(center, size);
            region.children[octant as usize] = Box::new(OctNode::new_empty(subdivide_center, subdivide_size));
        }

        OctNode::Region(region)
    }

    fn new_leaf(center: Vec3, size: f32) -> OctNode {
        OctNode::Leaf(
            Leaf {
                bounding_box: BoundingBox::new(center, size),
                primitives: vec![],
            }
        )
    }

    fn new_populated_leaf(center: Vec3, size: f32, primitive: BoundingBox) -> OctNode {
        OctNode::Leaf(
            Leaf {
                bounding_box: BoundingBox::new(center, size),
                primitives: vec![primitive],
            }
        )
    }

    fn new_empty(center: Vec3, size: f32) -> Self {
        OctNode::Empty(
            Empty {
                bounding_box: BoundingBox::new(center, size),
            }
        )
    }

    fn insert(oct_node: &mut OctNode, primitive: BoundingBox, min_size: f32) {
        match oct_node {
            OctNode::Empty(empty) => { 
                let center = empty.bounding_box.position;
                let size = empty.bounding_box.size;
                *oct_node = OctNode::new_populated_leaf(center, size, primitive);
            },

            OctNode::Leaf(leaf) => {
                if Leaf::is_not_subdividable(leaf, min_size) {
                    leaf.primitives.push(primitive);
                }
                else {
                    let center = leaf.bounding_box.position;
                    let size = leaf.bounding_box.size;
                    let primitives = leaf.primitives.clone();
                    *oct_node = OctNode::new_region(center, size); 

                    for leaf_primitive in primitives {
                        OctNode::insert(oct_node, leaf_primitive, min_size);
                    }

                    OctNode::insert(oct_node, primitive, min_size);
                }
            },

            OctNode::Region(region) => Region::insert(region, primitive, min_size),
        }
    }

    pub fn get_potential_collisions(oct_node: &mut OctNode, potential_collisions: &mut Vec<usize>, primitive: BoundingBox) {
        match oct_node {
            OctNode::Region(region) => {
                Region::get_potential_collisions(region, potential_collisions, primitive);
            },
            OctNode::Leaf(leaf) => {
                Leaf::get_potential_collisions(leaf, potential_collisions, primitive);
            },
            OctNode::Empty(empty) => {
            },
        }
    }

    pub fn print(oct_node: &OctNode) {
        match oct_node {
            OctNode::Region(region) => {
                for primitive in &region.primitives {
                    println!("Body: {}", primitive.body_id);
                }
                for (i, child_node) in region.children.iter().enumerate() {
                    OctNode::print(child_node);
                }
            },
            OctNode::Leaf(leaf) => {
                for primitive in &leaf.primitives {
                    if !BoundingBox::is_intersecting(*primitive, leaf.bounding_box) {
                        println!("Hello world");
                    }
                }
            },
            OctNode::Empty(empty) => {
            },
        }
    }

    pub fn get_subdivisions(oct_node: &OctNode, subdivisions: &mut Vec<BoundingBox>) {
        match oct_node {
            OctNode::Region(region) => {
                subdivisions.push(region.bounding_box);
                for child_node in &region.children {
                    OctNode::get_subdivisions(child_node, subdivisions);
                }
            },
            OctNode::Leaf(leaf) => {
                subdivisions.push(leaf.bounding_box);
            },
            OctNode::Empty(empty) => {
                return;
            },
        }
    }
}

impl Region {

    fn insert(region: &mut Region, primitive: BoundingBox, min_size: f32) {
        let point = primitive.position;
        let octant = region.get_octant(point);
        OctNode::insert(&mut region.children[octant as usize], primitive, min_size);
    }

    fn get_potential_collisions(region: &mut Region, potential_collisions: &mut Vec<usize>, primitive: BoundingBox) {
        if BoundingBox::is_intersecting(primitive, region.bounding_box) {
            for region_primitive in &region.primitives {
                if BoundingBox::is_intersecting(*region_primitive, primitive) {
                    potential_collisions.push(primitive.body_id);
                }
            }
            for child_node in &mut region.children {
                OctNode::get_potential_collisions(child_node, potential_collisions, primitive);
            }
        }
    }

    fn get_octant(&self, point: Vec3) -> Octant {
        let (mid_x, mid_y, mid_z) = (
            self.bounding_box.position.x, 
            self.bounding_box.position.y, 
            self.bounding_box.position.z
        );

        let Vec3 {x, y, z,} = point;

        if  x <= mid_x {
            if y <= mid_y {
                if z <= mid_z {
                    Octant::BotLeftBack
                }
                else {
                    Octant::BotLeftFront
                }
            }
            else {
                if z <= mid_z {
                    Octant::TopLeftBack
                }
                else {
                    Octant::TopLeftFront
                }
            }
        }
        else {
            if y <= mid_y {
                if z <= mid_z {
                    Octant::BotRightBack
                }
                else {
                    Octant::BotRightFront
                }
            }
            else {
                if z <= mid_z {
                    Octant::TopRightBack
                }
                else {
                    Octant::TopRightFront
                }
            }
        }
    }
}

impl Leaf {
    fn insert(leaf: &mut Leaf, primitive: BoundingBox) {
        leaf.primitives.push(primitive);
    }

    fn get_potential_collisions(leaf: &mut Leaf, potential_collisions: &mut Vec<usize>, primitive: BoundingBox) {
        //FUCKING REWRITE THIS FUNCTION PLEASE DEAR GOD THIS SUCKS

            let mut found = false;
        if BoundingBox::is_intersecting(primitive, leaf.bounding_box) {

            for leaf_primitive in &leaf.primitives {
                if leaf_primitive.body_id != primitive.body_id {
                    potential_collisions.push(leaf_primitive.body_id);
                }

            /*
            let mut self_index = 0;
            for (i, leaf_primitive) in leaf.primitives.iter().enumerate() {
                if primitive.body_id == leaf_primitive.body_id {
                    self_index = i;
                    found = true;
                    break;
                }
            }
            if found {
                leaf.primitives.remove(self_index);
            }
            */


                /*
                if BoundingBox::is_intersecting(primitive, *leaf_primitive) {
                    println!("not dupplicate {}, {}", primitive.body_id, leaf_primitive.body_id);
                }
                */
            }

        }
    }

    fn is_not_subdividable(leaf: &Leaf, min_size: f32) -> bool {
        leaf.bounding_box.size < min_size / 2.0
    }
}

#[derive(Copy, Clone)]
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

impl Octant {
    fn iter() -> impl Iterator<Item = Octant> {
        [
            Octant::TopLeftFront,
            Octant::TopLeftBack,
            Octant::TopRightFront,
            Octant::TopRightBack,
            Octant::BotLeftFront,
            Octant::BotLeftBack,
            Octant::BotRightFront,
            Octant::BotRightBack,
        ].iter().copied()
    }

    fn print(&self) {
        match(self) {
            Octant::TopLeftFront => {
                println!("TopLeftFront");
            },
            Octant::TopLeftBack => {
                println!("TopLeftBack");
            },
            Octant::TopRightFront => {
                println!("TopRightFront");
            },
            Octant::TopRightBack => {
                println!("TopRightBack");
            },
            Octant::BotLeftFront => {
                println!("BotLeftFront");
            },
            Octant::BotLeftBack => {
                println!("BotLeftBack");
            },
            Octant::BotRightFront => {
                println!("BotRightFront");
            },
            Octant::BotRightBack => {
                println!("BotRightBack");
            },
        }
    }

    fn get_center(&self, mut center: Vec3, mut size: f32) -> Vec3 {
        size = size / 2.0;

        match self {
            Octant::TopLeftFront => {
                center.x -= size / 2.0;
                center.y += size / 2.0;
                center.z += size / 2.0;
            },
            Octant::TopLeftBack => {
                center.x -= size / 2.0;
                center.y += size / 2.0;
                center.z -= size / 2.0;
            },
            Octant::TopRightFront => {
                center.x += size / 2.0;
                center.y += size / 2.0;
                center.z += size / 2.0;
            },
            Octant::TopRightBack => {
                center.x += size / 2.0;
                center.y += size / 2.0;
                center.z -= size / 2.0;
            },
            Octant::BotLeftFront => {
                center.x -= size / 2.0;
                center.y -= size / 2.0;
                center.z += size / 2.0;
            },
            Octant::BotLeftBack => {
                center.x -= size / 2.0;
                center.y -= size / 2.0;
                center.z -= size / 2.0;
            },
            Octant::BotRightFront => {
                center.x += size / 2.0;
                center.y -= size / 2.0;
                center.z += size / 2.0;
            },
            Octant::BotRightBack => {
                center.x += size / 2.0;
                center.y -= size / 2.0;
                center.z -= size / 2.0;
            },
        }

        center
    }
}




