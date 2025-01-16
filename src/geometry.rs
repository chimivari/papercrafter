use std::{fmt::Display, ops::Sub};

use wavefront_obj::obj;

#[derive(Debug, Clone, Copy)]
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

    pub fn cross(&self, o: Self) -> Self {
        Vertex::new(
            self.y * o.z - self.z * o.y, 
            self.z * o.x - self.x * o.z, 
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn get_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, o: Self) -> Self::Output {
        Self::new(
            self.x - o.x, 
            self.y - o.y, 
            self.z - o.z,
        )
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
        .map(|&v| Vertex::from(v))
        .collect()
}

#[derive(Debug)]
pub struct Triangle{
    pub p1: Vertex,
    pub p2: Vertex,
    pub p3: Vertex,
    pub plane_equation: (f64, f64, f64, f64),
}

impl Triangle {
    pub fn new(p1: Vertex, p2: Vertex, p3: Vertex) -> Self {
        Triangle {
            p1,
            p2,
            p3,
            plane_equation: Self::calculate_plante_equation(p1, p2, p3),
        }
    }

    /// Calculate plane equation ax + by + cz + d = 0 -> (a, b, c, d)
    fn calculate_plante_equation(p1: Vertex, p2: Vertex, p3: Vertex) -> (f64, f64, f64, f64) {
        let p1p2 = p2 - p1;
        let p1p3 = p3 - p1;
        let (a, b, c) = p1p2.cross(p1p3).get_tuple();
        let d = -(a * p1.x + b * p1.y + c * p1.z);
        (a, b, c, d)
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.p1, self.p2, self.p3)
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
}

