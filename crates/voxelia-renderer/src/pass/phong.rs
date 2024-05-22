//! The [PhongPass] is a struct that represents the phong pass of to render objects.

use wgpu::{BindGroupLayout, IndexFormat};

use crate::{
    globals::Globals,
    instance::InstanceRaw,
    model::{Material, Mesh},
    pipeline,
    renderer::Renderer,
    texture,
    vertex::ModelVertex,
};

use super::Pass;

/// The primary pass for rendering the entire thing.
pub struct PhongPass {
    pub depth_texture: texture::Texture,
    pub render_pipeline: pipeline::Pipeline,
    pub texture_bind_group_layout: BindGroupLayout,
}

impl PhongPass {
    pub fn new(renderer: &Renderer, globals: &Globals) -> Self {
        let device = &renderer.device;
        let config = &renderer.config;

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "Depth Texture");
        let texture_bind_group_layout = texture::default_texture_bind_group_layout(device);

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &globals.camera.layout],
            push_constant_ranges: &[],
        });

        let vertex = pipeline::include_shader!(device, "../shaders/shader.wgsl");
        let fragment = pipeline::include_shader!(device, "../shaders/shader.wgsl");

        let render_pipeline = pipeline::Pipeline::new(
            &device,
            &config,
            layout,
            vertex,
            &[ModelVertex::desc(), InstanceRaw::desc()],
            fragment,
        );

        Self {
            depth_texture,
            render_pipeline,
            texture_bind_group_layout,
        }
    }

    /// Resizes the depth buffer of the [Phase]
    pub fn resize(&mut self, renderer: &Renderer) {
        self.depth_texture = texture::Texture::create_depth_texture(
            &renderer.device,
            &renderer.config,
            "Depth Texture",
        );
    }
}

impl Pass for PhongPass {
    fn draw(
        &mut self,
        renderer: &Renderer,
        materials: &[Material],
        meshes: &[Mesh],
        globals: &Globals,
    ) -> Result<(), wgpu::SurfaceError> {
        // Gives a surface to create a new frame of.
        let output = renderer.surface.get_current_texture()?;

        // Describes a new texture view so it can handle textures from the surface.
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a new encoder so we can just send commands to the GPU in a queue.
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline.pipeline);

            for mesh in meshes {
                let material = &materials[mesh.material_id as usize];

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

                render_pass.set_bind_group(0, &material.bind_group, &[]);
                render_pass.set_bind_group(1, &globals.camera.group, &[]);

                render_pass.set_vertex_buffer(1, mesh.instance_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);

                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..mesh.num_instances);
            }
        }

        // Submits the commands to the GPU.
        renderer.queue.submit(std::iter::once(encoder.finish()));

        // Presents the frame to the screen.
        output.present();

        Ok(())
    }
}
