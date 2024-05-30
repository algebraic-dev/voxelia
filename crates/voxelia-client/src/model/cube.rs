//! Definition of a Cube in the renderer realm. It's used in order to generate chunks and other structures
//! based on Cubes, its not widely used by itself.

use voxelia_renderer::{ModelIndex, ModelVertex};
use super::vertex;

pub const FRONT : usize = 0;
pub const BACK : usize = 1;
pub const LEFT : usize = 2;
pub const RIGHT : usize = 3;
pub const TOP : usize = 4;

#[rustfmt::skip]
pub const VERTICES: &[ModelVertex] = &[
    vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]),
    vertex!([1.0, -1.0, 1.0], [0.0, 1.0]), 
    vertex!([1.0, 1.0, 1.0], [0.0, 0.0]),  
    vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]), 
    vertex!([-1.0, -1.0, -1.0], [1.0, 1.0]),
    vertex!([1.0, -1.0, -1.0], [0.0, 1.0]), 
    vertex!([1.0, 1.0, -1.0], [0.0, 0.0]),  
    vertex!([-1.0, 1.0, -1.0], [1.0, 0.0]), 
    vertex!([-1.0, 1.0, -1.0], [0.0, 0.0]), 
    vertex!([-1.0, -1.0, -1.0], [0.0, 1.0]),
    vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]), 
    vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]),  
    vertex!([1.0, 1.0, -1.0], [0.0, 0.0]), 
    vertex!([1.0, 1.0, 1.0], [1.0, 0.0]),  
    vertex!([1.0, -1.0, 1.0], [1.0, 1.0]), 
    vertex!([1.0, -1.0, -1.0], [0.0, 1.0]),
    vertex!([-1.0, 1.0, 1.0], [0.0, 0.0]), 
    vertex!([1.0, 1.0, 1.0], [1.0, 0.0]),  
    vertex!([1.0, 1.0, -1.0], [1.0, 1.0]), 
    vertex!([-1.0, 1.0, -1.0], [0.0, 1.0]),
    vertex!([-1.0, -1.0, 1.0], [0.0, 0.0]), 
    vertex!([-1.0, -1.0, -1.0], [0.0, 1.0]),
    vertex!([1.0, -1.0, -1.0], [1.0, 1.0]), 
    vertex!([1.0, -1.0, 1.0], [1.0, 0.0]),  
];

#[rustfmt::skip]
pub const INDICES: &[ModelIndex] = &[
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

pub fn face(number: usize) -> (&'static [ModelVertex], &'static [ModelIndex]) {
    (
        &VERTICES[number * 4..(number + 1) * 4],
        &INDICES[number * 6..(number + 1) * 6],
    )
}
