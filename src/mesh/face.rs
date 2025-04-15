use nalgebra::{Matrix3, Vector3 as Vertex};
use std::ops;

pub struct Face {
    pub id: usize,
    pub unfolded: bool,
    pub normal: Vertex<f64>,
    pub vertices: Vec<Vertex<f64>>,
    pub neighbourhoud: Vec<usize>,
}

impl Face {
    const EPSILON: f64 = 1e-5;

    pub fn new(id: usize, vertices: Vec<Vertex<f64>>) -> Self {
        Self {
            id,
            unfolded: false,
            normal: Vertex::cross(&(vertices[1] - vertices[0]), &(vertices[2] - vertices[0])),
            vertices,
            neighbourhoud: vec![],
        }
    }

    pub fn rotate(&mut self, r: Matrix3<f64>) {
        for i in 0..self.vertices.len() {
            self[i] = r * self[i];
        }
        self.normal = Vertex::cross(&(self[1] - self[0]), &(self[2] - self[0]));
    }

    /// The face is parallel with (xz) plane
    pub fn is_flat(&self) -> bool {
        let normal = *self.normal.normalize();
        (normal.x.abs() < Self::EPSILON) &&
        ((1. - (normal.y).abs()).abs() < Self::EPSILON) &&
        (normal.z.abs() < Self::EPSILON)
    }
}

impl ops::Index<usize> for Face {
    type Output = Vertex<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl ops::IndexMut<usize> for Face {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

impl ops::SubAssign<Vertex<f64>> for Face {
    fn sub_assign(&mut self, rhs: Vertex<f64>) {
        for i in 0..self.vertices.len() {
            self.vertices[i] -= rhs;
        }
    }
}

impl ops::AddAssign<Vertex<f64>> for Face {
    fn add_assign(&mut self, rhs: Vertex<f64>) {
        for i in 0..self.vertices.len() {
            self.vertices[i] += rhs;
        }
    }
}
