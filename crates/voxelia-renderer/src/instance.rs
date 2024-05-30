//! Instance of a mesh.

/// A instance is a object of a mesh that is duplicated in the scene with position and rotation
/// changes.
pub struct ModelInstance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl ModelInstance {
    pub fn new(position: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>) -> Self {
        ModelInstance {
            position,
            rotation
        }
    }

    pub fn from_position(position: cgmath::Vector3<f32>) -> Self {
        Self {
            position,
            rotation: cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl From<ModelInstance> for InstanceRaw {
    fn from(value: ModelInstance) -> Self {
        value.to_raw()
    }
}

/// The raw data of a instance that is sent to the GPU.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}

impl ModelInstance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model = self.translation() * self.rotation();

        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }

    fn translation(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_translation(self.position)
    }

    fn rotation(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from(self.rotation)
    }
}

impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,

            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}