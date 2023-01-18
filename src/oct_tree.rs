use crate::vec3::Vec3;
use crate::body::Body;
use core::slice::Iter;

pub enum OctNode<'a> {
    Region(Region<'a>),
    Leaf(Leaf<'a>),
    Empty,
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
};

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
    /*

    pub fn find<'a, 'b>(oct_tree: &'a mut OctTree<'b>, body: &'b Body) {
        OctNode::find(&mut oct_tree.root, body);

    }
    */
}

impl OctNode<'_> {
    fn new_region(center: Vec3, size: f32) -> Self {
        let mut region = Region {
            center: center,
            size: size,
            children: [
                //This is fucking stupid but I don't want Body to have the copy trait tbh so eh idk
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
                Box::new(OctNode::Empty(Vec3::NULL_VEC, 0.0)),
            ],
            bodies: Vec::new(),
        };

        for octant in Octant::iter() {
            let subdivide_size = size / 8.0;
            let subdivide_center = octant.get_center(region);
            region.children[octant as usize] = Box::new(new_empty(subdivide_center, subdivide_size));
        }

        OctNode::Region(
            region: region,
        )

    }

    fn new_leaf() -> Self {
        OctNode::Region(
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
            OctNode::Empty => {},
        }
    }
    /*

    fn find<'a, 'b>(oct_node: &'a mut OctNode<'b>, body: &'b Body) {
        match oct_node {
            OctNode::Region(region) => Region::find(region, body),
            OctNode::Empty => oct_node
        }

    }
    */
}

impl Region<'_> {
    fn insert<'a, 'b>(region: &'a mut Region<'b>, body: &'b Body, min_size: f32) {
        if region.bodies.empty() {
            regions.bodies.push(body);
            return;
        }
        let point = body.transform.position;
        let octant = region.get_octant(point) as usize;
        OctNode::insert(&mut region.children[octant], body, min_size);
    }

    /*
    fn find<'a, 'b>(region: &'a mut Region<'b>, body: &'b Body) {
        for oct_node in region {
            if OctNode::contains(oct_node, body) {
                OctNode::find(oct_node)
            }

        }
    }
    */

    fn is_not_subdividable(region: &Region, min_size: f32) -> bool {
        region.size < min_size / 8.0
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

}

impl Empty<'_> {
    fn insert<'a, 'b>(empty: &'a mut Leaf<'b>, body: &'b Body) {
        if Region::is_not_subdividable(region, min_size) {
            empty = OctNode::new_leaf(body);
            return;
        }
        empty = OctNode::new_region(empty.center, empty.size); 
        empty.bodies.push(body);
    }

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

impl Octant {
    fn iter() -> Iter<'static, Octant> {
        static Octants: [Octant, 8] = [
            TopLeftFront,
            TopLeftBack,
            TopRightFron,
            TopRightBack,
            BotLeftFront,
            BotLeftBack,
            BotRightFron,
            BotRightBack,
        ];
        Octants.iter();
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




