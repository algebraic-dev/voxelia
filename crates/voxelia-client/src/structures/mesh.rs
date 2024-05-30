//! Definition of meshes and dynamic meshes that can be used as components.

use specs::{Component, VecStorage};
use voxelia_renderer::model::{Mesh, Model};

#[derive(Component)]
#[storage(VecStorage)]
pub struct DynamicMesh {
    pub data: Mesh,
    pub model: Model,
}
