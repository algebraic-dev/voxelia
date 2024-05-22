//! Components related to chunks, chunk rendering and stuff.

use specs::Join;
use specs::{Entities, ReadStorage, System, WriteExpect, WriteStorage};
use voxelia_engine::chunk::Chunk;
use voxelia_engine::events::Created;
use voxelia_renderer::model::{chunk, MaterialId};

use crate::graphics::Graphics;
use crate::mesh::DynamicMesh;

/// Receives a ChunkCreated event and then creates a rendered thing for it.
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Graphics>,
        ReadStorage<'a, Chunk>,
        WriteStorage<'a, Created>,
        WriteStorage<'a, DynamicMesh>,
    );

    fn run(&mut self, (entities, info, chunk, mut created, mut renders): Self::SystemData) {
        let entities_to_remove: Vec<_> = (&entities, &chunk, &created)
            .join()
            .map(|(entity, chunk, _)| (entity, chunk))
            .collect();

        for (entity, chunk) in entities_to_remove {
            created.remove(entity);
            let data = chunk::to_mesh(chunk.data, MaterialId(0), &info.renderer);
            renders.insert(entity, DynamicMesh { data }).unwrap();
        }
    }
}
