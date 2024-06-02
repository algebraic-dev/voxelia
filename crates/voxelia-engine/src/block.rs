use std::ops::{Add, Sub};

use crate::chunk;

#[derive(Debug)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl BlockPosition {
    pub const fn new(x: i64, y: i64, z: i64) -> BlockPosition {
        BlockPosition {
            x,
            y,
            z,
        }
    }

    pub fn is_out(&mut self) -> bool {
        self.x < 0 || self.x >= chunk::CHUNK_WIDTH as i64 ||
        self.y < 0 || self.y >= chunk::CHUNK_HEIGHT as i64 ||
        self.z < 0 || self.z >= chunk::CHUNK_LENGTH as i64
    }
}

impl Add for BlockPosition {
    type Output = BlockPosition;

    fn add(self, rhs: Self) -> Self::Output {
        BlockPosition {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}


impl Add for &BlockPosition {
    type Output = BlockPosition;

    fn add(self, rhs: Self) -> Self::Output {
        BlockPosition {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl Sub for BlockPosition {
    type Output = BlockPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        BlockPosition {
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}