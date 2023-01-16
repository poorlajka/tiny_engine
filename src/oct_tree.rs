struct OctNode {
    children: [OctNode; 8],
    bodies: Vec<Body>,
    size: f32,
    is_leaf: bool,
}



impl OctNode {
    fn new(region_space: f32, min_subdivide: f32) {
        let mut node = OctNode {
            children:
        }

        if region_space > min_subdivide / 8.0 {
            //Not a leaf node
            child_1 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // top right front corner

            child_2 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // top right back corner

            child_3 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // bottom right front corner

            child_4 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // bottom right back corner 

            child_5 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // top left front corner

            child_6 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // top left back corner

            child_7 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // bottom left front corner

            child_8 = OctNode::new(total_space_volume / 8.0, min_subdivide_volume) // bottom left back corner
        }
        else {
            //Leaf node
            node.is_leaf = true;
        }

        node
    }

    fn insert(&mut self, bounding_volume: Sphere) {


    }

    fn get_neighbours(&mut self, bounding_volume: Sphere) {

    }
}



