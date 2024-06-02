//! Definition of a Cube in the renderer realm. It's used in order to generate chunks and other structures
//! based on Cubes, its not widely used by itself.

use voxelia_engine::block::BlockPosition;
use voxelia_renderer::{ModelIndex, ModelVertex};
use super::vertex;

pub const FRONT : usize = 0;
pub const BACK : usize = 1;
pub const LEFT : usize = 2;
pub const RIGHT : usize = 3;
pub const TOP : usize = 4;
pub const BOTTOM : usize = 5;

pub const FRONT_FACE: BlockPosition = BlockPosition::new(0, 0, 1);
pub const BACK_FACE: BlockPosition = BlockPosition::new(0, 0, -1);
pub const LEFT_FACE: BlockPosition = BlockPosition::new(-1, 0, 0);
pub const RIGHT_FACE: BlockPosition = BlockPosition::new(1, 0, 0);
pub const TOP_FACE: BlockPosition = BlockPosition::new(0, 1, 0);
pub const BOTTOM_FACE: BlockPosition = BlockPosition::new(0, -1, 0);

pub const FACE_DISPLACEMENT : [BlockPosition; 6] = [
    FRONT_FACE,
    BACK_FACE,
    LEFT_FACE,
    RIGHT_FACE,
    TOP_FACE,
    BOTTOM_FACE,
];

#[rustfmt::skip]
pub const VERTICES: &[ModelVertex] = &[
    vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]),
    vertex!([1.0, -1.0, 1.0], [0.0, 1.0]), 
    vertex!([1.0, 1.0, 1.0], [0.0, 0.0]),  
    vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]), 
 
    vertex!([1.0, 1.0, -1.0], [0.0, 0.0]),
    vertex!([1.0, -1.0, -1.0], [0.0, 1.0]),
    vertex!([-1.0, -1.0, -1.0], [1.0, 1.0]),
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
pub const INDICES: &[ModelIndex] = &[0, 1, 2, 3, 0, 2];

pub fn face(number: usize) -> &'static [ModelVertex] {
    &VERTICES[number * 4..(number + 1) * 4]
}
