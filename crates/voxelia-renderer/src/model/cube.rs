use crate::instance::ModelInstance;
use crate::model::ModelVertex;
use crate::model::Mesh;
use crate::renderer::Renderer;

use cgmath::Rotation3;

macro_rules! vertex {
    ($position:expr, $tex_coords:expr) => {
        ModelVertex {
            position: $position,
            tex_coords: $tex_coords,
        }
    };
}

// Translate VERTICES to use vertex!
fn vertex() -> Vec<ModelVertex> {
    vec![
        // front
        vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]), // 0
        vertex!([1.0, -1.0, 1.0], [0.0, 1.0]),  // 1
        vertex!([1.0, 1.0, 1.0], [0.0, 0.0]),   // 2
        vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]),  // 3
        // back
        vertex!([-1.0, -1.0, -1.0], [1.0, 1.0]), // 4
        vertex!([1.0, -1.0, -1.0], [0.0, 1.0]),  // 5
        vertex!([1.0, 1.0, -1.0], [0.0, 0.0]),   // 6
        vertex!([-1.0, 1.0, -1.0], [1.0, 0.0]),  // 7
        // left
        vertex!([-1.0, 1.0, -1.0], [0.0, 0.0]),  // 7
        vertex!([-1.0, -1.0, -1.0], [0.0, 1.0]), // 4
        vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]),  // 0
        vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]),   // 3
        // right
        vertex!([1.0, 1.0, -1.0], [0.0, 0.0]),  // 6
        vertex!([1.0, 1.0, 1.0], [1.0, 0.0]),   // 2
        vertex!([1.0, -1.0, 1.0], [1.0, 1.0]),  // 1
        vertex!([1.0, -1.0, -1.0], [0.0, 1.0]), // 5
        // top
        vertex!([-1.0, 1.0, 1.0], [0.0, 0.0]),  // 3
        vertex!([1.0, 1.0, 1.0], [1.0, 0.0]),   // 2
        vertex!([1.0, 1.0, -1.0], [1.0, 1.0]),  // 6
        vertex!([-1.0, 1.0, -1.0], [0.0, 1.0]), // 7
        // bottom
        vertex!([-1.0, -1.0, 1.0], [0.0, 0.0]),  // 0
        vertex!([-1.0, -1.0, -1.0], [0.0, 1.0]), // 4
        vertex!([1.0, -1.0, -1.0], [1.0, 1.0]),  // 5
        vertex!([1.0, -1.0, 1.0], [1.0, 0.0]),   // 1
    ]
}

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    // Front
    0, 1, 2, 3, 0, 2,
    // Back
    6, 5, 4, 6, 4, 7,
    // left
    8, 9, 10, 11, 8, 10,
    // Right
    12, 13, 14, 15, 12, 14,
    // Top
    16, 17, 18, 19, 16, 18,
    // Bottom
    20, 21, 22, 23, 20, 22,
];

pub struct Cube;

impl Cube {
    pub fn to_mesh(&self, renderer: &Renderer) -> Mesh {
        Mesh::from_vertex(renderer, "Cube".to_owned(), &vertex(), INDICES, &[ModelInstance {
            position: cgmath::Vector3::new(0.0, 0.0, 4.0),
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        }], 0)
    }
}