use wgpu::RenderPipeline;
use winit::window::Window;

use crate::{
    camera::Camera,
    mesh::{self, shapes::pentagon::Pentagon, *},
    pass_data::Globals,
    pipeline,
    renderer::Renderer,
    texture::{self, Texture},
};

pub struct State {
    pub renderer: Renderer,
    pub camera: Camera,
    pub global: Globals,
    pub texture: Texture,
    pub pentagon: mesh::Mesh,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let renderer = Renderer::new(window).await;

        let img = include_bytes!("img.png");
        let texture = Texture::from_bytes(&renderer.device, &renderer.queue, img, "ata").unwrap();

        let mut global = Globals::new(&renderer.device, &texture);

        global.add_pipeline(pipeline::Pipeline::primary(
            &renderer.device,
            &renderer.config,
            &global,
        ));

        Self {
            camera: Camera::new(renderer.size),
            global,
            pentagon: Pentagon.to_mesh(&renderer.device),
            texture,
            renderer,
        }
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        self.global.camera.data.update_view_proj(&self.camera);

        self.renderer.queue.write_buffer(
            &self.global.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.global.camera.data]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.pentagon, &self.global)
    }
}
