use std::{collections::HashMap, fmt::Display};
use nalgebra::{Matrix3, Vector3 as Vertex};

use face::Face;
use neighbour::Neighbour;

use crate::triangle::Triangle;


mod face;
mod neighbour;


pub struct Mesh {
    pub faces: Vec<Face>,
    pub neighbours: Vec<Neighbour>,
}

impl Mesh {
    pub fn new(triangles: &Vec<Triangle>, vertices: &Vec<Vertex<f64>>) -> Self {
        let mut faces = vec![];
        let mut neighbours = vec![];
        Self::init(triangles, vertices, &mut faces, &mut neighbours);
        Self {
            faces,
            neighbours,
        }
    }

    /// Compute all [`faces`] and face's [`neighbours`]
    /// from triangles and vertices
    pub fn init(
        triangles: &Vec<Triangle>,
        vertices: &Vec<Vertex<f64>>,
        faces: &mut Vec<Face>,
        neighbours: &mut Vec<Neighbour>
    ) {
        let mut hmap: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for i in 0..triangles.len() {
            let t = &triangles[i];
            
            let pairs = [
                if t.0 < t.1 {(t.0, t.1)} else {(t.1, t.0)},
                if t.2 < t.1 {(t.2, t.1)} else {(t.1, t.2)},
                if t.0 < t.2 {(t.0, t.2)} else {(t.2, t.0)},
            ];

            for pair in pairs {
                if let Some(edge) = hmap.get_mut(&pair) {
                    edge.push(i);
                }
                else {
                    let mut edge = Vec::with_capacity(2);
                    edge.push(i);
                    hmap.insert(pair, edge);
                }
            }
            // Compute face from triangle
            let face = Face::new(
                i, 
                vec![vertices[t.0], vertices[t.1], vertices[t.2]]
            );
            faces.push(face);
        }
        // Compute neighbours from hash_map
        for kvp in hmap.iter() {
            let pair= kvp.0;
            let indexes = kvp.1;
            if indexes.len() == 2 {
                let t1 = &triangles[indexes[0]];
                let t2 = &triangles[indexes[1]];

                let f1 = &faces[indexes[0]];
                let f2 = &faces[indexes[1]];

                let f1_futherest =
                    if t1.0 != pair.0 && t1.0 != pair.1 {0}
                    else if t1.1 != pair.0 && t1.1 != pair.1 {1}
                    else {2};
                let f2_futherest =
                    if t2.0 != pair.0 && t2.0 != pair.1 {0}
                    else if t2.1 != pair.0 && t2.1 != pair.1 {1}
                    else {2};

                let n1 = Neighbour::new(f2.id, f1_futherest, f2_futherest);
                let n2 = Neighbour::new(f1.id, f2_futherest, f1_futherest);

                faces[0].neighbourhoud.push(neighbours.len());
                neighbours.push(n1);
                faces[1].neighbourhoud.push(neighbours.len());
                neighbours.push(n2);
            }
        }
    }


    pub fn init_unfolding(&mut self, face_id: usize) {
        if self.faces[face_id].unfolded {return;}

        /* Transpose mesh */
        let v0 = self.faces[face_id][0];
        for i in 0..self.faces.len() {
            let f = &self.faces[i];
            if !f.unfolded {
                let f = &mut self.faces[i];
                *f -= v0;
            }
        }

        /* Rotate mesh */
        let wish = Vertex::new(0., 1., 0.);
        let normal = self.faces[face_id].normal;
        // Compute rotation around x
        let sub_x = Vertex::new(0., normal.y, normal.z);
        if sub_x != Vertex::zeros() {
            let mut theta_x = Vertex::angle(&sub_x, &wish);
            if Vertex::dot(&Vertex::z(), &sub_x) > 0. {
                theta_x = -theta_x;
            }
            let rx = {
                let sin = theta_x.sin();
                let cos = theta_x.cos();
                Matrix3::new(
                    1., 0., 0.,
                    0., cos, -sin,
                    0., sin, cos
                )
            };
            // Apply rotation
            for i in 0..self.faces.len() {
                if !self.faces[i].unfolded {
                    self.faces[i].rotate(rx);
                }
            }
        }
        // Compute rotation around z
        let normal = self.faces[face_id].normal;
        let sub_z = Vertex::new(normal.x, normal.y, 0.);
        if sub_z != Vertex::zeros() {
            let mut theta_z = -Vertex::angle(&sub_z, &wish);
            if Vertex::dot(&Vertex::x(), &sub_z) > 0. {
                theta_z = -theta_z;
            }
            let rz = {
                let sin = theta_z.sin();
                let cos = theta_z.cos();
                Matrix3::new(
                    cos, -sin, 0.,
                    sin, cos, 0.,
                    0., 0., 1.
                )
            };
            // Apply rotation
            for i in 0..self.faces.len() {
                if !self.faces[i].unfolded {
                    self.faces[i].rotate(rz);
                }
            }
        }
    }
}

impl Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.faces.len() == 0 {return write!(f, "[]");}
        let _ = write!(f, "[\n");
        for face in &self.faces {
            let _ = write!(f, "\t{}\n", face);
        }
        write!(f, "]")
    }
}
