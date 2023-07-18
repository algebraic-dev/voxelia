//! Vertex types for models.

use crate::mesh::vertex::Vertex;

/// A primitive vertex that contains position, a normal vector, and a texture coordinate. Usually
/// used for 3D objects.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl ModelVertex {
    pub const DESC: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2
    ];
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::DESC,
        }
    }
}
