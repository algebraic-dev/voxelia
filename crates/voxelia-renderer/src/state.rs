use winit::window::Window;

use crate::{
    camera::Camera,
    mesh::{self, shapes::cube::Cube, *},
    pass_data::Globals,
    pipeline,
    renderer::Renderer,
    texture::Texture,
};

pub struct State {
    pub renderer: Renderer,
    pub camera: Camera,
    pub global: Globals,
    pub texture: Texture,
    pub pentagon: mesh::Mesh,
    pub start: std::time::Instant,
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
            pentagon: Cube.to_mesh(&renderer.device),
            texture,
            start: std::time::Instant::now(),
            renderer,
        }
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
        self.camera.aspect = self.renderer.config.width as f32 / self.renderer.config.height as f32;
    }

    pub fn input(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        self.camera.eye.x = self.start.elapsed().as_secs_f32().sin() * 5.0;
        self.camera.eye.y = self.start.elapsed().as_secs_f32().cos() * 5.0;
        self.camera.eye.z = self.start.elapsed().as_secs_f32().cos() * 5.0;

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
