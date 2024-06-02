use cgmath::Vector3;
use voxelia_engine::block::BlockPosition;

pub trait Absolute {
    fn to_absolute(&self) -> Vector3<f32>;

    fn to_slice(&self) -> [f32; 3] {
        self.to_absolute().into()
    }
}

impl Absolute for BlockPosition {
    fn to_absolute(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32 * 2.0, self.y as f32 * 2.0, self.z as f32 * 2.0)
    }
}