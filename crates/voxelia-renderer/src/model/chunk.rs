//! Prototype of the Chunk mesh

use crate::instance::ModelInstance;
use crate::model::Mesh;
use crate::renderer::Renderer;
use super::cube;
use super::MaterialId;
use cgmath::Rotation3;

pub fn to_mesh<const X: usize, const Y: usize, const Z: usize>(data: [[[u8; Z]; Y]; X], material_id: MaterialId, renderer: &Renderer) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for x in 0..X {
        for y in 0..Y {
            for z in 0..Z {
                let data = data[x][y][z];
                if data == 1 {
                    let coord = [x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0];
                    indices.extend(cube::INDICES.iter().map(|x| x + vertices.len() as u16));
                    vertices.extend(cube::VERTICES.iter().map(|v| v.add(coord)));
                }
            }
        }
    }

    Mesh::from_vertex(renderer, "Chunk".to_owned(), &vertices, &indices, &[ModelInstance {
        position: cgmath::Vector3::new(0.0, 0.0, 0.0),
        rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
    }], material_id)
}