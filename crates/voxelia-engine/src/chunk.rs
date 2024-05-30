//! Definition of Chunks

use specs::{Component, VecStorage};

use crate::{Plugin, WorldBuilder};

pub const CHUNK_WIDTH: usize = 4;
pub const CHUNK_HEIGHT: usize = 4;
pub const CHUNK_LENGTH: usize = 4;

pub struct ChunkPosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/// Chunk component that stores the information about a chunk.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    pub data: [u8; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_LENGTH],
}

impl Chunk {
    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.data[z + y * CHUNK_WIDTH + x * CHUNK_WIDTH * CHUNK_HEIGHT]
    }
}

/// Plugin for rendering and creating chunks.
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Chunk>()
    }
}