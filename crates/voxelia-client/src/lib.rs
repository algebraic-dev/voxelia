use structures::{graphics::Graphics, mesh::DynamicMesh};
use systems::{chunk::ChunkRenderSystem, render::RendererSystem};
use voxelia_engine::Plugin;

pub mod structures;
pub mod model;
pub mod systems;
pub mod position;

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
