//! This module introduces the [Vertex] trait that is used to describe vertex objects for vertex
//! buffers.

/// A vertex is a point in the space and is contains some information about the shape, and texture
/// of the object, this is the reason it returns a [VertexBufferLayout].
pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
