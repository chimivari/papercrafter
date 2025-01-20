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

pub struct Face {
    pub points: (usize, usize, usize),
    pub quadric: Matrix4<f64>,
}

impl Face {
    fn new(points: (usize, usize, usize), vertices: &mut (Vertex, Vertex, Vertex)) -> Self {
        Self {
            points,
            quadric: Self::compute_quadric(vertices),
        }
    }

    fn compute_quadric(vertices: &mut(Vertex, Vertex, Vertex)) -> Matrix4<f64> {
        let p1 = vertices.0.position;
        let p2 = vertices.1.position;
        let p3 = vertices.1.position;

        let n = (&p2 - p1).cross(&(&p3 - &p1)).normalize();
            let (a, b, c, d) = (n.x, n.y, n.z, -n.dot(&p1));

        let k =Matrix4::new(
            a*a, a*b, a*c, a*d, 
            b*a, b*b, b*c, b*d, 
            c*a, c*b, c*c, c*d, 
            d*a, d*b, d*c, d*d
        );

        vertices.0.quadric += k;
        vertices.1.quadric += k;
        vertices.2.quadric += k;

        k
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub edges: BinaryHeap<Edge>,
    pair_length_threshold_sqrd: f64,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, faces: Vec<(usize, usize, usize)>, t: f64) -> Self {
        let faces: Vec<Face> = faces
            .iter()
            .map(|&f| 
                Face::new(f, &mut (vertices[f.0].clone(), vertices[f.1].clone(), vertices[f.2].clone()))
            )
            .collect();

        let mut mesh = Mesh {
            vertices,
            faces,
            edges: BinaryHeap::new(),
            pair_length_threshold_sqrd: t * t,
        };
        
        mesh.compute_valid_edges();

        mesh
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

        for face in &self.faces {
            let (p1, p2, p3) = face.points;
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

    fn get_common_vertices(edge: &Edge, face: &Face) -> Vec<usize> {
        let mut vertices = Vec::new();
        let (p1, p2) = (edge.p1, edge.p2);
        let (f1, f2, f3) = face.points;
        if p1 == f1 || p1 == f2 || p1 == f3 {
            vertices.push(p1);
        }
        if p2 == f1 || p2 == f2 || p2 == f3 {
            vertices.push(p2);
        }
        vertices
    }

    fn get_unshared_vertices(edge: &Edge, face: &Face) -> Vec<usize> {
        let mut vertices = Vec::new();
        let (p1, p2) = (edge.p1, edge.p2);
        let (f1, f2, f3) = face.points;
        if f1 != p1 && f1 != p2 {
            vertices.push(f1);
        }
        if f2 != p1 && f2 != p2 {
            vertices.push(f2);
        }
        if f3 != p1 && f3 != p2 {
            vertices.push(f3);
        }
        vertices
    }

    pub fn simplify(&mut self, target_ratio: f64) {
        let target_vertices = (self.vertices.len() as f64 * target_ratio).ceil() as usize;
        while self.edges.len() > 0 && self.vertices.len() > target_vertices {
            if let Some(e) = self.edges.pop() {
                let p1 = e.p1;
                let p2 = e.p2;
                let new_vertex = Vertex::new(e.optimal_pos);

                let mut i = 0;
                while i < self.faces.len() {
                    let face = &self.faces[i];
                    let unshares = Self::get_unshared_vertices(&e, face);
                    
                    // If the triangle contains the edge
                    if unshares.len() == 1 {
                        // Remove the quadric error of the face from the unshared vertex
                        let vi = unshares[0];
                        self.vertices[vi].quadric -= face.quadric;
                        // Remove the triangle from the mesh
                        self.faces.remove(i);
                        continue;
                    }
                    // If the triangle shares a single vertex with the edge 
                    else if unshares.len() == 2 {
                        // Remove the quadric face error from the unshared vertices
                        for &vi in &unshares {
                            self.vertices[vi].quadric -= face.quadric;
                        }
                        // Build the new face with the contraction
                        // The contraction vertex will replace the p1 index
                        let new_points = (unshares[0], unshares[1], p1);
                        // The new face quadric error will be addded in the constructor
                        // on the face's vertices 
                        let new_face = Face::new(
                            new_points, 
                            &mut (
                                self.vertices[new_points.0].clone(),
                                self.vertices[new_points.1].clone(),
                                new_vertex.clone(),
                            )
                        );
                        self.faces[i] = new_face;
                    }

                    // Change the face indexes because of edge contraction
                    // The contraction index will be in p1
                    let face = &mut self.faces[i];
                    if face.points.0 > p1 {
                        face.points.0 -= 1;
                    }
                    if face.points.1 > p1 {
                        face.points.1 -= 1;
                    }
                    if face.points.2 > p1 {
                        face.points.2 -= 1;
                    }

                    i += 1;
                }

                // p1 -> contraction
                self.vertices[p1] = new_vertex;
                // Remove p2
                self.vertices.remove(p2);

                self.compute_valid_edges();

                println!("still {} vertices", self.vertices.len());
            }
        }
    }
}