use nalgebra::Vector3;

#[derive(Debug)]
pub struct Triangle {
    pub v: [usize; 3],
    pub attr: usize,
    pub material: i32,
    pub uvs: [Vector3<f64>; 3],
}

impl Triangle {
    
}