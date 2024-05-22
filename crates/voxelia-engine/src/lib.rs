//! The entrypoint for the Voxelia engine. This thing exposes a way to create a simulation of a voxelia
//! world and provide ways to interact with the world in a high-level way.

pub mod core;
pub mod chunk;
pub mod events;

pub use core::*;

use chunk::Chunk;
use events::Created;

pub struct VoxeliaPlugin;

impl Plugin for VoxeliaPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Chunk>();
        world.with_component::<Created>()
    }
}