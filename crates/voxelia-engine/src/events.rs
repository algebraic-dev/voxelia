//! Definition of components for events

use specs::{Component, NullStorage};

/// Component for things that were created right now
#[derive(Component)]
#[storage(NullStorage)]
pub struct Created;
