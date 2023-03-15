use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub position: Vec3,
    pub size: f32,
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
    pub max_z: f32,
    pub min_z: f32,
    pub body_id: usize,
}

impl BoundingBox {
    pub fn new(position: Vec3, size: f32,) -> BoundingBox {
        BoundingBox {
            position: position,
            size: size,
            max_x: position.x + size / 2.0,
            min_x: position.x - size / 2.0,
            max_y: position.y + size / 2.0,
            min_y: position.y - size / 2.0,
            max_z: position.z + size / 2.0,
            min_z: position.z - size / 2.0,
            body_id: 0,
        }
    }

    pub fn is_intersecting(box_a: BoundingBox, box_b: BoundingBox) -> bool {
        if box_a.min_x > box_b.max_x {
            return false;
        }
        if box_a.max_x < box_b.min_x {
            return false
        }
        if box_a.min_y > box_b.max_y {
            return false;
        }
        if box_a.max_y < box_b.min_y {
            return false
        }
        if box_a.min_z > box_b.max_z {
            return false;
        }
        if box_a.max_z < box_b.min_z {
            return false
        }

        true
    }
}
