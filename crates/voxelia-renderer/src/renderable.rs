//! Module for things that can be drawn on the screen.

type Global = crate::pass_data::Globals;

pub trait Renderable {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, global: &'a Global);
}
