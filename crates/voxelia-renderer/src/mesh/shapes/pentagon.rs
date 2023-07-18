use crate::mesh::{model::vertex::ModelVertex, Mesh, ToMesh};
use wgpu::util::DeviceExt;

pub const VERTICES: &[ModelVertex] = &[
    ModelVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    },
    ModelVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    },
    ModelVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    },
    ModelVertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    },
    ModelVertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    },
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct Pentagon;

impl ToMesh for Pentagon {
    fn to_mesh(&self, device: &wgpu::Device) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ModelVertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            name: "Pentagon".to_owned(),
            vertex_buffer,
            index_buffer,
            num_elements: INDICES.len() as u32,
        }
    }
}
