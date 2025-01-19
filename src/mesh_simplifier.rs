use std::collections::{BinaryHeap, HashSet};

use nalgebra::{Matrix4, Vector3, Vector4};


#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f64>,
    pub quadric: Matrix4<f64>,
}

impl Vertex {
    pub fn new(position: Vector3<f64>) -> Self {
        Self {
            position,
            quadric: Matrix4::zeros(),
        }
    }
}


pub struct Edge {
    pub p1: usize,
    pub p2: usize,
    pub cost: f64,
    pub optimal_pos: Vector3<f64>,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<(usize, usize, usize)>,
    pub edges: BinaryHeap<Edge>,
    pair_length_threshold_sqrd: f64,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, faces: Vec<(usize, usize, usize)>, t: f64) -> Self {
        let mut mesh = Mesh {
            vertices,
            faces,
            edges: BinaryHeap::new(),
            pair_length_threshold_sqrd: t * t,
        };
        
        mesh.compute_quadrics();
        mesh.compute_valid_edges();

        mesh
    }

    fn compute_quadrics(&mut self) {
        for v in &mut self.vertices {
            v.quadric = Matrix4::zeros();
        }

        for &(v1, v2, v3) in &self.faces {
            let p1 = self.vertices[v1].position;
            let p2 = self.vertices[v2].position;
            let p3 = self.vertices[v3].position;

            let n = (&p2 - p1).cross(&(&p3 - &p1)).normalize();
            let (a, b, c, d) = (n.x, n.y, n.z, -n.dot(&p1));

            let k = Matrix4::new(
                a*a, a*b, a*c, a*d, 
                b*a, b*b, b*c, b*d, 
                c*a, c*b, c*c, c*d, 
                d*a, d*b, d*c, d*d
            );

            self.vertices[v1].quadric += &k;
            self.vertices[v2].quadric += &k;
            self.vertices[v3].quadric += &k;
        }
    }

    fn compute_edge_contraction(&self, v1: usize, v2: usize) -> Edge {
        let p1 = self.vertices[v1].clone();
        let p2 = self.vertices[v2].clone();

        let q = p1.quadric + p2.quadric;

        let m = Matrix4::new(
            q[(0, 0)], q[(0, 1)], q[(0, 2)], q[(0, 3)], 
            q[(0, 1)], q[(1, 1)], q[(1, 2)], q[(1, 3)], 
            q[(0, 2)], q[(1, 2)], q[(2, 2)], q[(2, 3)], 
            0., 0., 0., 1.);
        
        let p3 = if let Some(m) = m.try_inverse() {
            m * Vector4::new(0., 0., 0., 1.)
        } else {
            let mid = (p1.position + p2.position) * 0.5;
            Vector4::new(mid.x, mid.y, mid.z, 1.)
        };

        let cost = (p3.transpose() * q * p3)[0];

        Edge {
            p1: v1,
            p2: v2,
            cost,
            optimal_pos: Vector3::new(p3.x, p3.y, p3.z),
        }
    }

    fn compute_valid_edges(&mut self) {
        let mut pairs = HashSet::new();

        for &(p1, p2, p3) in &self.faces {
            let pair1 = if p1 < p2 {(p1, p2)} else {(p2, p1)};
            let pair2 = if p3 < p2 {(p3, p2)} else {(p2, p3)};
            let pair3 = if p1 < p3 {(p1, p3)} else {(p3, p1)};

            pairs.insert(pair1);
            pairs.insert(pair2);
            pairs.insert(pair3);
        }

        if self.pair_length_threshold_sqrd > 0. {
            for i in 0..(self.vertices.len() - 1) {
                for j in (i + 1)..(self.vertices.len()) {
                    let pair = (i, j);
                    if !pairs.contains(&pair) {
                        let p1 = self.vertices[i].position;
                        let p2 = self.vertices[j].position;
                        if (&p1 - &p2).norm_squared() < self.pair_length_threshold_sqrd {
                            pairs.insert(pair);
                        }
                    }
                }
            } 
        }

        self.edges = BinaryHeap::new();
        for (v1, v2) in pairs {
            let edge = self.compute_edge_contraction(v1, v2);
            self.edges.push(edge);
        }
    }

    fn face_contains(face: (usize, usize, usize), p: usize) -> bool {
        face.0 == p || face.1 == p || face.2 == p
    }

    pub fn simplify(&mut self, target_ratio: f64) {
        let target_vertices = (self.vertices.len() as f64 * target_ratio).ceil() as usize;
        while self.edges.len() > 0 && self.vertices.len() > target_vertices {
            if let Some(e) = self.edges.pop() {
                let p1 = e.p1;
                let p2 = e.p2;

                let mut i = 0;
                while i < self.faces.len() {
                    let &face = &self.faces[i];
                    if Self::face_contains(face, p1) && Self::face_contains(face, p2) {
                        self.faces.remove(i);
                    }
                    else {
                        if face.0 == p2 {
                            self.faces[i].0 = p1;
                        }
                        else if face.1 == p2 {
                            self.faces[i].1 = p1;
                        }
                        else if face.2 == p2 {
                            self.faces[i].2 = p1;
                        }

                        if face.0 > p1 {
                            self.faces[i].0 -= 1;
                        }
                        if face.1 > p1 {
                            self.faces[i].1 -= 1;
                        }
                        if face.2 > p1 {
                            self.faces[i].2 -= 1;
                        }

                        i += 1;
                    }
                }

                // p1 -> contraction
                self.vertices[p1] = Vertex::new(e.optimal_pos);
                // Remove p2
                self.vertices.remove(p2);

                self.compute_quadrics();
                self.compute_valid_edges();
                println!("still {} vertices", self.vertices.len());
            }
        }
    }
}