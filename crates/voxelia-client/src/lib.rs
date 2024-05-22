use chunk::{Chunk, ChunkRenderSystem, ChunkRenderer, Created};
use graphics::Graphics;
use specs::{System, WriteExpect, WriteStorage};
use voxelia_engine::Plugin;
use voxelia_renderer::pass::Pass;
use specs::Join;

pub mod graphics;
pub mod chunk;

/// Renders all the meshes.
pub struct RendererSystem;

impl<'a> System<'a> for RendererSystem {
    type SystemData = (WriteExpect<'a, Graphics>, WriteStorage<'a, ChunkRenderer>);

    fn run(&mut self, (info, renders): Self::SystemData) {
        let meshes = renders.join().map(|x| &x.data).collect::<Vec<_>>();
        info.pass
            .draw(&info.renderer, &info.materials, &meshes, &info.globals)
            .unwrap();
    }
}

pub struct RendererPlugin {
    pub info: Graphics,
}

impl Plugin for RendererPlugin {
    fn setup(self, world: &mut voxelia_engine::WorldBuilder) {
        world.with_component::<Chunk>();
        world.with_component::<Created>();
        world.with_component::<ChunkRenderer>();

        world.with_resource(self.info);
        world.with_system(ChunkRenderSystem, "chunk render system", &[]);
        world.with_system(RendererSystem, "renderer system", &[]);
    }
}