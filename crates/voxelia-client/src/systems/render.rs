use specs::{System, WriteExpect, WriteStorage, Join};
use voxelia_renderer::Pass;

use crate::structures::{graphics::Graphics, mesh::DynamicMesh};

/// Renders all the meshes.
pub struct RendererSystem;

impl<'a> System<'a> for RendererSystem {
    type SystemData = (WriteExpect<'a, Graphics>, WriteStorage<'a, DynamicMesh>);

    fn run(&mut self, (mut info, renders): Self::SystemData) {
        info.update_camera();
        let meshes = renders.join().map(|x| &x.data).collect::<Vec<_>>();
        info.pass
            .draw(&info.renderer, &info.materials, &meshes, &info.globals)
            .unwrap();
    }
}