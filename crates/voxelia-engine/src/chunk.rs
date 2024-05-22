//! Definition of Chunks

use specs::{Component, VecStorage};

use crate::{Plugin, WorldBuilder};

/// Chunk component that stores the information about a chunk.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    pub data: [[[u8; 4]; 4]; 4],
}

/// Plugin for rendering and creating chunks.
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Chunk>()
    }
}