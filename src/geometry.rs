use std::{fmt::Display, ops::Sub};

use ndarray::{arr2, Array2, Dim, OwnedRepr};
use wavefront_obj::obj;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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

#[derive(Debug, Clone)]
pub struct Triangle{
    pub p1: Vertex,
    pub p2: Vertex,
    pub p3: Vertex,
    pub fund_err_quad: Array2<f64>,
}

impl Triangle {
    pub fn new(p1: Vertex, p2: Vertex, p3: Vertex) -> Self {
        Triangle {
            p1,
            p2,
            p3,
            fund_err_quad: Self::calculate_fundamental_error_quadric(p1, p2, p3),
        }
    }

    /// Calculate plane equation ax + by + cz + d = 0 -> (a, b, c, d)
    fn calculate_fundamental_error_quadric(p1: Vertex, p2: Vertex, p3: Vertex) -> Array2<f64> {
        let p1p2 = p2 - p1;
        let p1p3 = p3 - p1;
        let (a, b, c) = p1p2.cross(p1p3).get_tuple();
        let d = -(a * p1.x + b * p1.y + c * p1.z);
        arr2(&[
            [a*a, a*b, a*c, a*d],
            [a*b, b*b, b*c, b*d],
            [a*c, b*c, c*c, c*d],
            [a*d, b*d, c*d, d*d],
        ])
    }

    /// Returns if p1, p2 or p3 == p
    pub fn contains_point(&self, p: &Vertex) -> bool {
        self.p1 == *p || self.p2 == *p || self.p3 == *p
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
    pub vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, vertices: Vec<Vertex>) -> Self {
        Self {
            triangles,
            vertices,
        }
    }
}

