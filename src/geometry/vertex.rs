use crate::math::{symetric_matrix::SymetricMatrix, vec3f::Vec3F};

#[derive(Debug)]
pub struct Vertex {
    p: Vec3F,
    tstart: i32,
    tcount: i32,
    q: SymetricMatrix,
    border: i32,
}

impl Vertex {
    pub fn new(p: Vec3F) -> Self {
        Self {
            p,
            tstart: 0,
            tcount: 0,
            q: SymetricMatrix::zeros(),
            border: 0,
        }
    }
}