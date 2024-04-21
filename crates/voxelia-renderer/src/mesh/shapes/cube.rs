use crate::mesh::{model::vertex::ModelVertex, Mesh, ToMesh};
use wgpu::util::DeviceExt;

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

impl ToMesh for Cube {
    fn to_mesh(&self, device: &wgpu::Device) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ModelVertex Buffer"),
            contents: bytemuck::cast_slice(vertex().as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            name: "Cube".to_owned(),
            vertex_buffer,
            index_buffer,
            num_elements: INDICES.len() as u32,
        }
    }
}
