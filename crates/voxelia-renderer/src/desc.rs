//! Description of a vertex. This is used to describe the layout of a vertex in a vertex buffer.
//! The main purpose of this trait is to avoid having to write the same code over and over again.

use wgpu::VertexAttribute;
pub trait Desc<const T: usize>: Sized {
    const ATTRIBUTES: [VertexAttribute; T];

    /// Returns the description of the vertex buffer layout.
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
