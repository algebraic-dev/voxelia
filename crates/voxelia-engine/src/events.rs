//! Definition of components for events

use specs::{Component, NullStorage};

use crate::{Plugin, WorldBuilder};

/// Component for things that were created right now
#[derive(Component)]
#[storage(NullStorage)]
pub struct Created;

/// Plugin for event handling
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Created>()
    }
}
