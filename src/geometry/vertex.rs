use nalgebra::Vector3;

#[derive(Debug)]
pub struct Vertex {
    pub p: Vector3<f64>,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            p: Vector3::new(x, y, z),
        }
    }
}