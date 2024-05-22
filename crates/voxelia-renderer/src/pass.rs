//! This module defines a [Pass] that is a trait to render something with a [RenderPipeline].

use crate::{model::{Material, Mesh}, renderer::Renderer};

pub mod phong;

/// Shared behaviour of being something that is able to render thigns to the screen
pub trait Pass {
    fn draw(
        &mut self,
        renderer: &Renderer,
        materials: &[Material],
        meshes: &[Mesh],
    ) -> Result<(), wgpu::SurfaceError>;
}
