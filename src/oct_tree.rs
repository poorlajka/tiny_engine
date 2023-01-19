use crate::vec3::Vec3;
use crate::body::Body;
use core::slice::Iter;

pub enum OctNode<'a> {
    Region(Region<'a>),
    Leaf(Leaf<'a>),
    Empty(Empty),
}

pub struct Region<'a> {
    pub center: Vec3,
    pub size: f32,
    pub children: [Box<OctNode<'a>>; 8],
    pub bodies: Vec<&'a Body>,
}

pub struct Leaf<'a> {
    pub bodies: Vec<&'a Body>,
}

pub struct Empty {
    pub center: Vec3,
    pub size: f32,
}

pub struct OctTree<'a> {
    root: OctNode<'a>,
    min_size: f32,
}

impl OctTree<'_> {
    pub fn new(center: Vec3, size: f32, min_size: f32) -> Self {
        OctTree {
            root: OctNode::new_region(center, size),
            min_size: min_size,
        }
    }

    pub fn insert<'a, 'b>(oct_tree: &'a mut OctTree<'b>, body: &'b Body) {
        OctNode::insert(&mut oct_tree.root, body, oct_tree.min_size);
    }

    pub fn get_potential_collisions<'a, 'b>(oct_tree: &'a mut OctTree<'b>, potential_collisions: &mut Vec<usize>, body: &Body) {
        OctNode::get_potential_collisions(&mut oct_tree.root, potential_collisions, body);
    }
}

impl OctNode<'_> {
    fn new_region(center: Vec3, size: f32) -> Self {
        let mut region = Region {
            center: center,
            size: size,
            children: [
                //This is fucking stupid but I don't want Body to have the copy trait tbh so eh idk
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
                Box::new(Self::new_empty(Vec3::NULL_VEC, 0.0)),
            ],
            bodies: Vec::new(),
        };

        for octant in Octant::iter() {
            let subdivide_size = size / 8.0;
            let subdivide_center = octant.get_center(center, size);
            region.children[octant as usize] = Box::new(OctNode::new_empty(subdivide_center, subdivide_size));
        }

        OctNode::Region(region)

    }

    fn new_leaf() -> Self {
        OctNode::Leaf(
            Leaf {
                bodies: Vec::new(),
            }
        )
    }

    fn new_empty(center: Vec3, size: f32) -> Self {
        OctNode::Empty(
            Empty {
                center: center,
                size: size,
            }
        )
    }

    fn insert<'a, 'b>(oct_node: &'a mut OctNode<'b>, body: &'b Body, min_size: f32) {
        match oct_node {
            OctNode::Region(region) => Region::insert(region, body, min_size),
            OctNode::Leaf(leaf) => Leaf::insert(leaf, body),
            OctNode::Empty(empty) => {
                if Empty::is_not_subdividable(empty, min_size) {
                    *oct_node = OctNode::new_leaf()
                }
                else {
                    *oct_node = OctNode::new_region(empty.center, empty.size); 
                }
                OctNode::insert(oct_node, body, min_size);
            },
        }
    }

    pub fn get_potential_collisions<'a, 'b>(oct_node: &'a mut OctNode<'b>, potential_collisions: &mut Vec<usize>, body: &Body) {
        match oct_node {
            OctNode::Region(region) => Region::get_potential_collisions(&mut oct_tree.root, potential_collisions, body);
            OctNode::Leaf(leaf) => Leaf::get_potential_collisions(&mut oct_tree.root, potential_collisions, body);
            OctNode::Empty(empty) => {};
        }
    }
}

impl Region<'_> {
    fn insert<'a, 'b>(region: &'a mut Region<'b>, body: &'b Body, min_size: f32) {
        if region.bodies.is_empty() {
            region.bodies.push(body);
            return;
        }
        let point = body.transform.position;
        let octant = region.get_octant(point) as usize;
        OctNode::insert(&mut region.children[octant], body, min_size);
    }

    fn get_potential_collisions<'a, 'b>(region: &'a mut Region<'b>, potential_collisions: &mut Vec<usize>, body: &Body) {
        if aabb_intersect(body, region) {
            for body in region.bodies {
                if aabb_intersect() {
                    potential_collisions.push(body);
                }
            }
            for child_node in region.children {
                OctNode::get_potential_collisions(child_node, potential_collisions);
            }
        }
    }

    fn get_octant(&self, point: Vec3) -> Octant {
        let (mid_x, mid_y, mid_z) = (self.center.x, self.center.y, self.center.z);
        let Vec3 {x, y, z,} = point;

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

impl Leaf<'_> {
    fn insert<'a, 'b>(leaf: &'a mut Leaf<'b>, body: &'b Body) {
        leaf.bodies.push(body);
    }
    fn get_potential_collisions<'a, 'b>(region: &'a mut Leaf<'b>, potential_collisions: &mut Vec<usize>, body: &Body) {
        for primitive in leaf.bodies {
            if aabb_intersect(body, primitive) {
                potential_collisions.push(body);
            }
        }
    }
}

impl Empty {
    fn is_not_subdividable(empty: &Empty, min_size: f32) -> bool {
        empty.size < min_size / 8.0
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

    fn get_center(&self, mut center: Vec3, mut size: f32) -> Vec3 {
        size = size / 8.0;

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




