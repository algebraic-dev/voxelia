//! Module for bind_groups, layout groups and things that needs to be passed between render passes.

use crate::{
    camera::CameraUniform,
    pipeline::Pipeline,
    texture::{self, Texture},
};
use wgpu::util::DeviceExt;

/// Uniform data that is passed to the shaders.
pub struct Uniform<T> {
    pub group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
    pub data: T,
}

impl<T: bytemuck::Pod> Uniform<T> {
    pub fn new(device: &wgpu::Device, layout: wgpu::BindGroupLayout, data: T, label: &str) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Buffer", label)),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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

pub struct TextureUniform {
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
}

impl TextureUniform {
    pub fn new(device: &wgpu::Device, texture: &texture::Texture) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        Self { layout, group }
    }
}

// Data that is global to the whole renderer.
pub struct Globals {
    pub camera: Uniform<CameraUniform>,
    pub texture: TextureUniform,
    pub pipelines: Vec<Pipeline>,
}

impl Globals {
    pub fn new(device: &wgpu::Device, texture: &Texture) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera bind group layout"),
        });

        Self {
            camera: Uniform::new(device, layout, Default::default(), "camera"),
            texture: TextureUniform::new(device, texture),
            pipelines: Vec::new(),
        }
    }

    pub fn add_pipeline(&mut self, pipeline: Pipeline) {
        self.pipelines.push(pipeline);
    }
}
