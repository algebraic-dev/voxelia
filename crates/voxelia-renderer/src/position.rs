use cgmath::Vector3;

pub trait Absolute {
    fn to_absolute(&self) -> Vector3<f32>;
}

pub type GlobalPosition = Vector3<f32>;