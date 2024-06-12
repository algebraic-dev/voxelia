use specs::{System, ReadStorage, WriteStorage};

#[derive(Debug, Clone, Copy)]
struct AABB {
    min: cgmath::Vector3<f32>,
    max: cgmath::Vector3<f32>,
}

impl AABB {
    fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct BoundingBox(AABB);

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, BoundingBox>,
        WriteStorage<'a, Transform>, // Assuming you have a Transform component for positions
    );

    fn run(&mut self, (bounding_boxes, mut transforms): Self::SystemData) {
        for (bbox_a, transform_a) in (&bounding_boxes, &mut transforms).join() {
            for (bbox_b, transform_b) in (&bounding_boxes, &transforms).join() {
                if bbox_a == bbox_b {
                    continue;
                }

                if bbox_a.0.intersects(&bbox_b.0) {
                    // Handle collision response
                    // Example: Simple position adjustment
                    let overlap_x = (bbox_a.0.max.x - bbox_b.0.min.x).min(bbox_b.0.max.x - bbox_a.0.min.x);
                    let overlap_y = (bbox_a.0.max.y - bbox_b.0.min.y).min(bbox_b.0.max.y - bbox_a.0.min.y);
                    let overlap_z = (bbox_a.0.max.z - bbox_b.0.min.z).min(bbox_b.0.max.z - bbox_a.0.min.z);

                    if overlap_x < overlap_y && overlap_x < overlap_z {
                        transform_a.position.x -= overlap_x;
                    } else if overlap_y < overlap_x && overlap_y < overlap_z {
                        transform_a.position.y -= overlap_y;
                    } else {
                        transform_a.position.z -= overlap_z;
                    }
                }
            }
        }
    }
}
