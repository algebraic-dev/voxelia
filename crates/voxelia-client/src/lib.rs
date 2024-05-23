use chunk::ChunkRenderSystem;
use graphics::Graphics;
use mesh::DynamicMesh;
use specs::{Join, System, WriteExpect, WriteStorage};
use voxelia_engine::Plugin;
use voxelia_renderer::pass::Pass;

pub mod chunk;
pub mod graphics;
pub mod mesh;

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

pub struct RendererPlugin {
    pub graphics: Graphics,
}

impl Plugin for RendererPlugin {
    fn setup(self, world: &mut voxelia_engine::WorldBuilder) {
        world.with_component::<DynamicMesh>();

        world.with_resource(self.graphics);
        world.with_system(ChunkRenderSystem, "chunk render system", &[]);
        world.with_system(RendererSystem, "renderer system", &[]);
    }
}
