//! Definition of Chunks

use specs::{Component, VecStorage};

use crate::{block::BlockPosition, Plugin, WorldBuilder};

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
    pub fn get(&self, position: &BlockPosition) -> u8 {
        let index = position.z as usize + position.y as usize * CHUNK_WIDTH + position.x as usize * CHUNK_WIDTH * CHUNK_HEIGHT;
        self.data[index as usize]
    }
}

/// Plugin for rendering and creating chunks.
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Chunk>()
    }
}