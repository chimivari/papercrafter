use crate::math::{symetric_matrix::SymetricMatrix, vec3f::Vec3F};



pub struct Vertex {
    p: Vec3F,
    tstart: i32,
    tcount: i32,
    q: SymetricMatrix,
    border: i32,
}