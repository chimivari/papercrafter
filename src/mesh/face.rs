use nalgebra::{zero, Matrix3, Vector2, Vector3 as Vertex};
use std::{fmt::Display, ops};

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
            round_v(&mut self[i]);
        }
        self.normal = Vertex::cross(&(self[1] - self[0]), &(self[2] - self[0]));
        round_v(&mut self.normal);
    }

    /// The face is parallel with (xz) plane
    pub fn is_flat(&self) -> bool {
        let normal = *self.normal.normalize();
        (normal.x.abs() < Self::EPSILON) &&
        ((1. - (normal.y).abs()).abs() < Self::EPSILON) &&
        (normal.z.abs() < Self::EPSILON)
    }

    /// Get axes for the SAT (works only when face is an unfolded triangle)
    fn get_axes(&self) -> [Vector2<f64>; 3] {
        let mut axes = [Vector2::<f64>::zeros(); 3];
        for i in 0..self.vertices.len() {
            let p1 = self[i];
            let p2 = self[if i + 1 == self.vertices.len() {0} else {i + 1}];
            let edge = p1 - p2;
            let normal = Vector2::new(edge.z, -edge.x);
            axes[i] = normal;
        }
        axes
    }

    /// Project the [``face``] on [``axis``] (works only when face is an unfolded triangle)
    fn project(&self, axis: &Vector2<f64>) -> (f64, f64) {
        let mut min = axis.dot(&self[0].remove_row(1));
        let mut max = min;
        for i in 1..self.vertices.len() {
            let p = axis.dot(&self[i].remove_row(1));
            if p < min {
                min = p;
            } 
            else if p > max {
                max = p;
            }
        }
        (min, max)
    }

    fn overlap(proj1: (f64, f64), proj2: (f64, f64)) -> bool {
        !(proj1.1 <= proj2.0 || proj2.1 <= proj1.0)
    }

    pub fn collide(&self, other: &Face) -> bool {
        let axes1 = self.get_axes();
        let axes2 = other.get_axes();
        for i in 0..axes1.len() {
            let axis = axes1[i];
            let p1 = self.project(&axis);
            let p2 = other.project(&axis);
            if !Self::overlap(p1, p2) {
                return false
            }
        }
        for i in 0..axes2.len() {
            let axis = axes2[i];
            let p1 = self.project(&axis);
            let p2 = other.project(&axis);
            if !Self::overlap(p1, p2) {
                return false
            }
        }
        true
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
            self[i] -= rhs;
            round_v(&mut self[i]);
        }
    }
}

impl ops::AddAssign<Vertex<f64>> for Face {
    fn add_assign(&mut self, rhs: Vertex<f64>) {
        for i in 0..self.vertices.len() {
            self[i] += rhs;
            round_v(&mut self[i]);
        }
    }
}

impl PartialEq<Face> for Face {
    fn eq(&self, other: &Face) -> bool {
        self.id == other.id
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vertices.len() == 0 {return write!(f, "[]");}
        let _ = write!(f, "[");
        for v in &self.vertices {
            let _ = write!(f, "({}, {}) ", v.x, v.z);
            // let _ = write!(f, "({:.3}, {:.2}, {:.3}) ", v.x, v.y, v.z);
        }
        write!(f, "]")
    }
}

fn round_v(v: &mut Vertex<f64>) {
    let x = (v.x * 1e8).round() / 1e8;
    let y = (v.y * 1e8).round() / 1e8;
    let z = (v.z * 1e8).round() / 1e8;
    v.x = x;
    v.y = y;
    v.z = z;
}
