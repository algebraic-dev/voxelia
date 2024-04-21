//! This is the module for [Mesh] creation. It's used to describe the shape of 2D and 3D objects.

use crate::{
    pass_data::{self},
    renderable::Renderable,
};

pub mod model;
pub mod shapes;
pub mod vertex;

/// A mesh is a collection of vertices and indices that describes a shape.
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

pub trait ToMesh {
    fn to_mesh(&self, device: &wgpu::Device) -> Mesh;
}

impl Renderable for Mesh {
    fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        global: &'a pass_data::Globals,
    ) {
        render_pass.set_pipeline(&global.pipelines[0].pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.set_bind_group(0, &global.texture.group, &[]);
        render_pass.set_bind_group(1, &global.camera.group, &[]);

        render_pass.draw_indexed(0..self.num_elements, 0, 0..1);
    }
}
