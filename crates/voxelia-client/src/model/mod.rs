pub mod cube;
pub mod chunk;

macro_rules! vertex {
    ($position:expr, $tex_coords:expr) => {
        ModelVertex {
            position: $position,
            tex_coords: $tex_coords,
        }
    };
}

pub(crate) use vertex;