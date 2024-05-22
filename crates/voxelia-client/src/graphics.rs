//! Module that contains a struct called [Graphics] that holds all information needed to render the
//! game.

use voxelia_renderer::{
    camera::{Camera, Projection}, globals::Globals, model::Material, pass::phong::PhongPass, renderer::Renderer, texture::Texture, PhysicalSize, Window
};

pub struct Graphics {
    pub renderer: Renderer,
    pub materials: Vec<Material>,
    pub globals: Globals,
    pub pass: PhongPass,
    pub projection: Projection,
    pub camera: Camera,
}

impl Graphics {
    pub async fn new(window: &Window) -> Graphics {
        let renderer = Renderer::new(window).await;
        let globals = Globals::new(&renderer);
        let phong = PhongPass::new(&renderer, &globals);

        let texture = Texture::from_bytes(&renderer, include_bytes!("b.jpeg"), "Bulacha").unwrap();
        let material = Material::from_texture(&renderer, texture, &phong.texture_bind_group_layout);

        let projection = Projection::new(renderer.size);
        let camera = Camera::new(
            (20.0, 20.0, 25.0),
            cgmath::Deg(-90.0 - 30.0),
            cgmath::Deg(-30.0),
        );

        let mut info = Graphics {
            renderer,
            materials: vec![material],
            globals,
            pass: phong,
            projection,
            camera,
        };

        info.update_camera();

        info
    }
    pub fn update_camera(&mut self) {
        self.globals
            .update_camera(&self.renderer, &self.camera, &self.projection)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.projection.aspect =
            self.renderer.config.width as f32 / self.renderer.config.height as f32;
        self.renderer.resize(size);
        self.update_camera();
        self.pass.resize(&self.renderer);
    }
}
