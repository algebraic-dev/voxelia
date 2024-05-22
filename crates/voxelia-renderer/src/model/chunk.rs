//! Prototype of the Chunk mesh

use super::cube;
use super::MaterialId;
use crate::instance::ModelInstance;
use crate::model::Mesh;
use crate::renderer::Renderer;
use cgmath::Rotation3;

pub fn to_mesh<const X: usize, const Y: usize, const Z: usize>(
    position: [f32; 3],
    data: [[[u8; Z]; Y]; X],
    material_id: MaterialId,
    renderer: &Renderer,
) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // TODO: Need to improve this code x.x

    let global = [
        position[0] as f32 * X as f32 * 2.0,
        position[1] as f32 * Y as f32 * 2.0,
        position[2] as f32 * Z as f32 * 2.0,
    ];

    for x in 0..X {
        for y in 0..Y {
            for z in 0..Z {
                let data = data[x][y][z];
                if data == 1 {
                    let coord = [
                        x as f32 * 2.0 + global[0],
                        y as f32 * 2.0 + global[1],
                        z as f32 * 2.0 + global[2],
                    ];
                    indices.extend(cube::INDICES.iter().map(|x| x + vertices.len() as u16));
                    vertices.extend(cube::VERTICES.iter().map(|v| v.add(coord)));
                }
            }
        }
    }

    Mesh::from_vertex(
        renderer,
        "Chunk".to_owned(),
        &vertices,
        &indices,
        &[ModelInstance {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
        }],
        material_id,
    )
}
