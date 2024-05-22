use crate::{
    camera::{Camera, CameraUniform, Projection},
    renderer::Renderer,
    uniform::Uniform,
};

/// Data that is global to the whole renderer.
pub struct Globals {
    pub camera: Uniform<CameraUniform>,
}

impl Globals {
    pub fn new(renderer: &Renderer) -> Globals {
        let layout = CameraUniform::layout(renderer);

        Globals {
            camera: Uniform::new(renderer, layout, Default::default(), "camera"),
        }
    }

    pub fn update_camera(&mut self, renderer: &Renderer, camera: &Camera, projection: &Projection) {
        self.camera.data.update_view_proj(camera, projection);

        renderer.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.data]),
        );
    }
}
