//! Components related to chunks, chunk rendering and stuff.

use specs::Join;
use specs::{Component, Entities, NullStorage, ReadStorage, System, VecStorage, WriteExpect,
    WriteStorage,
};
use voxelia_renderer::model::{chunk, MaterialId, Mesh};

use crate::graphics::Graphics;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ChunkRenderer {
    pub data: Mesh,
}

/// Chunk component that stores the information about a chunk.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    pub data: [[[u8; 4]; 4]; 4],
}

/// Component for things that were created right now
#[derive(Component)]
#[storage(NullStorage)]
pub struct Created;

/// Receives a ChunkCreated event and then creates a rendered thing for it.
pub struct ChunkRenderSystem;

impl<'a> System<'a> for ChunkRenderSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Graphics>,
        ReadStorage<'a, Chunk>,
        WriteStorage<'a, Created>,
        WriteStorage<'a, ChunkRenderer>,
    );

    fn run(&mut self, (entities, info, chunk, mut created, mut renders): Self::SystemData) {
        let entities_to_remove: Vec<_> = (&entities, &chunk, &created)
            .join()
            .map(|(entity, chunk, _)| (entity, chunk))
            .collect();

        for (entity, chunk) in entities_to_remove {
            created.remove(entity);
            let data = chunk::to_mesh(chunk.data, MaterialId(0), &info.renderer);
            renders.insert(entity, ChunkRenderer { data }).unwrap();
        }
    }
}
