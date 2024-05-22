//! This module defines a [Pass] that is a trait to render something with a [RenderPipeline].

use crate::renderer::Renderer;

pub mod phong;

/// Shared behaviour of being something that is able to render thigns to the screen
pub trait Pass {
    fn draw(
        &mut self,
        renderer: &Renderer,
    ) -> Result<(), wgpu::SurfaceError>;
}
