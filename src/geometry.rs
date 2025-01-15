use std::fmt::Display;

use wavefront_obj::obj;


pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn from(v: obj::Vertex) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn to_vertices(vertices: Vec<obj::Vertex>) -> Vec<Vertex> {
    vertices
        .iter()
        .map(|&v| Vertex::new(v.x, v.y, v.z))
        .collect()
}


pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);


pub struct Mesh {
    triangles: Vec<Triangle>,
}