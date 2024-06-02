//! This module handles the generation and update of chunk models. It includes functionality for
//! creating chunk models from data and computing their global positions within the game world.

use cgmath::Vector3;
use voxelia_engine::{block::BlockPosition, chunk::{Chunk, CHUNK_HEIGHT, CHUNK_LENGTH, CHUNK_WIDTH}, Position};
use voxelia_renderer::{MaterialId, Mesh, Model, ModelInstance, Renderer};

use crate::position::Absolute;

use super::cube;

pub struct ChunkModel {
    pub model: Model,
    pub mesh: Mesh,
}

impl ChunkModel {
    pub fn global_chunk_position(
        position: &Position,
    ) -> Vector3<f32> {
        Vector3::new(
            position.x * CHUNK_WIDTH as f32 * 2.0,
            position.y * CHUNK_HEIGHT as f32 * 2.0,
            position.z * CHUNK_LENGTH as f32 * 2.0,
        )
    }

    pub fn from_data(
        position: &Position,
        chunk: &Chunk,
        material_id: MaterialId,
        renderer: &Renderer,
    ) -> (Model, Mesh) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_LENGTH {
                    let coord = BlockPosition::new(x as i64, y as i64, z as i64);
                    if chunk.get(&coord) == 1 {
                        for i in 0..6 {
                            let displacement = &cube::FACE_DISPLACEMENT[i];
                            let mut neighbor_cube = &coord + displacement;
                            if neighbor_cube.is_out() || chunk.get(&neighbor_cube) == 0 {
                                let face_vertices = cube::face(i);
                                indices.extend(cube::INDICES.iter().map(|x| x + vertices.len() as u16));
                                vertices.extend(face_vertices.iter().map(|v| v.add(coord.to_slice())));
                            }
                        }
                    }
                }
            }
        }

        let mesh = Mesh::from_vertex(
            renderer,
            "Chunk".to_owned(),
            &vertices,
            &indices,
            &[ModelInstance::from_position(ChunkModel::global_chunk_position(position))],
            material_id,
        );

        (Model { vertices, indices }, mesh)
    }
}
