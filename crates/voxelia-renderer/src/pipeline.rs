//! Pipelines for rendering voxels.

use std::fs;

use crate::{
    mesh::{
        model::{instances::InstanceRaw, vertex::ModelVertex},
        vertex::Vertex,
    },
    pass_data::Globals,
    texture,
};

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        layout: wgpu::PipelineLayout,
        vertex_shader: wgpu::ShaderModule,
        vertex_layout: &[wgpu::VertexBufferLayout],
        fragment_shader: wgpu::ShaderModule,
    ) -> Self {
        Pipeline {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "vs_main",
                    buffers: vertex_layout,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }),
        }
    }

    pub fn primary(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        globals: &Globals,
    ) -> Self {
        let vertex = Self::load_shader(device, "crates/voxelia-renderer/src/shaders/shader.wgsl");
        let fragment = Self::load_shader(device, "crates/voxelia-renderer/src/shaders/shader.wgsl");

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&globals.texture.layout, &globals.camera.layout],
            push_constant_ranges: &[],
        });

        Self::new(
            device,
            config,
            layout,
            vertex,
            &[ModelVertex::desc(), InstanceRaw::desc()],
            fragment,
        )
    }

    pub fn load_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(fs::read_to_string(path).unwrap().into()),
        })
    }
}
