use crate::math::{vec2f::Vec2F, vec3f::Vec3F};


pub struct Triangle {
    pub vertices: [usize; 3],
    err: [f64; 4],
    deleted: i32,
    dirty: i32,
    attr: i32,
    n: Vec3F,
}

impl Triangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        Self {
            vertices: [v1, v2, v3],
            err: [0.; 4],
            deleted: 0,
            dirty: 0,
            attr: 0,
            n: Vec3F::new(0., 0., 0.),
        }
    }
}