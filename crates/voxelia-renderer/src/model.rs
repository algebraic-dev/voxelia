//! Defines what some things are like [Mesh] and [Material] that are extremely important for rendering
//! every [Model].

use crate::{instance::ModelInstance, renderer::Renderer, texture, vertex::{ModelIndex, ModelVertex}};
use wgpu::util::DeviceExt;

pub struct MaterialId(pub u32);

/// Defines a [Texture] with a BindGroup
pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn from_texture(
        renderer: &Renderer,
        diffuse_texture: texture::Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Material {
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: None,
            });

        Material {
            name: diffuse_texture.name.clone(),
            diffuse_texture,
            bind_group,
        }
    }
}

/// Stores the Buffers from drawing something.
pub struct Mesh {
    pub label: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub num_instances: u32,
    pub material_id: MaterialId,
}

impl Mesh {
    pub fn from_vertex(
        renderer: &Renderer,
        label: String,
        vertices: &[ModelVertex],
        indices: &[ModelIndex],
        instances: &[ModelInstance],
        material_id: MaterialId,
    ) -> Mesh {
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", label)),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", label)),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let instance_data = instances.iter().map(ModelInstance::to_raw).collect::<Vec<_>>();

        let instance_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label:Some(&format!("{:?} Instance Buffer", label)),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        Mesh {
            label,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            num_indices: indices.len() as u32,
            num_instances: instances.len() as u32,
            material_id,
        }
    }
}

/// A model here contains all the vertices and indices. Its used in order to update some mesh
pub struct Model {
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u16>,
}

impl Model {
    pub fn update_mesh(&self, queue: &wgpu::Queue, mesh: &mut Mesh) {
        queue.write_buffer(&mesh.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        queue.write_buffer(&mesh.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        mesh.num_indices = self.indices.len() as u32;
    } 
}