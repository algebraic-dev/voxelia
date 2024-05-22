//! The entrypoint for the Voxelia engine. This thing exposes a way to create a simulation of a voxelia
//! world and provide ways to interact with the world in a high-level way.

pub mod core;
pub mod chunk;
pub mod events;

pub use core::*;

use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position(pub [f32;3]);

/// Plugin for rendering and creating chunks.
pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Position>()
    }
}