//! Definition of Chunks

use specs::{Component, VecStorage};

/// Chunk component that stores the information about a chunk.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    pub data: [[[u8; 4]; 4]; 4],
}
