//! Definitions for passing data for shaders.

use wgpu::util::DeviceExt;

use crate::renderer::Renderer;

/// Uniform data that is passed to the shaders.
pub struct Uniform<T> {
    pub group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
    pub data: T,
}

impl<T: bytemuck::Pod> Uniform<T> {
    pub fn new(renderer: &Renderer, layout: wgpu::BindGroupLayout, data: T, label: &str) -> Self {
        let buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Buffer", label)),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some(&format!("{} Bind Group", label)),
        });

        Self {
            group,
            layout,
            buffer,
            data,
        }
    }
}