//! Components related to chunks, chunk rendering and stuff.

use specs::Join;
use specs::{Entities, ReadStorage, System, WriteExpect, WriteStorage};
use voxelia_engine::chunk::Chunk;
use voxelia_engine::events::Created;
use voxelia_engine::Position;
use voxelia_renderer::model::MaterialId;

use crate::structures::graphics::Graphics;
use crate::structures::mesh::DynamicMesh;
use crate::model::chunk;

/// Receives a ChunkCreated event and then creates a rendered thing for it.
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Graphics>,
        WriteStorage<'a, Created>,
        WriteStorage<'a, DynamicMesh>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Chunk>,
    );

    fn run(&mut self, (entities, info, mut created, mut renders, pos, chunk): Self::SystemData) {
        let entities_to_remove: Vec<_> = (&entities, &pos, &chunk, &created)
            .join()
            .map(|(entity, pos, chunk, _)| (entity, pos, chunk))
            .collect();

        for (entity, pos, chunk) in entities_to_remove {
            created.remove(entity);
            let (model, data) = chunk::ChunkModel::from_data(pos, chunk, MaterialId(0), &info.renderer);
            renders.insert(entity, DynamicMesh { data, model }).unwrap();
        }
    }
}
