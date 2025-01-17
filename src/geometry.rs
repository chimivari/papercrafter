use std::{collections::BinaryHeap, vec};

use nalgebra::{Matrix4, Vector3, Vector4};


#[derive(Debug)]
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

#[derive(Debug)]
pub struct Pair {
    pub v1: usize,
    pub v2: usize,
    pub error: f64,
    pub optimal_position: Vector4<f64>,
}

impl Pair {
    pub fn new(v1: usize, v2: usize) -> Self {
        Self {
            v1,
            v2,
            error: f64::INFINITY,
            optimal_position: Vector4::zeros(),
        }
    }

    pub fn compute_error(&mut self, quadric1: &Matrix4<f64>, quadric2: &Matrix4<f64>, pos1: &Vector3<f64>, pos2: &Vector3<f64>)  {
        let q = quadric1 + quadric2;
        let q_sub = q.fixed_view::<3, 3>(0, 0);
        let q_offset = q.fixed_view::<3, 1>(0, 3);

        if let Some(q_sub_inv) = q_sub.try_inverse() {
            let opt_pos = q_sub_inv * -q_offset;
            self.optimal_position = Vector4::new(opt_pos.x, opt_pos.y, opt_pos.z, 1.0);
        }
        else {
            let center = (pos1 + pos2) / 2.;
            self.optimal_position = Vector4::new(center.x, center.y, center.z, 1.);
        }

        self.error = (self.optimal_position.transpose() * q *self.optimal_position)[0];
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.error == other.error
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.error.partial_cmp(&self.error)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


pub struct SurfaceSimplifier {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<[usize; 3]>,
    pub heap: BinaryHeap<Pair>,
}

impl SurfaceSimplifier {
    pub fn new(vertices: Vec<Vector3<f64>>, faces: Vec<[usize; 3]>) -> Self {
        let vertices = vertices.into_iter().map(Vertex::new).collect();
        Self {
            vertices,
            faces,
            heap: BinaryHeap::new(),
        }
    }

    pub fn compute_quadric_matrices(&mut self) {
        for face in &self.faces {
            let v1 = self.vertices[face[0]].position;
            let v2 = self.vertices[face[1]].position;
            let v3 = self.vertices[face[2]].position;

            let normal = (v2 - v1).cross(&(v3 - v1)).normalize();
            let d = -normal.dot(&v1);
            let plane = Vector4::new(normal.x, normal.y, normal.z, d);
            let quadric = plane * plane.transpose();

            for &i in face {
                self.vertices[i].quadric += quadric;
            }
        }
    }

    pub fn initialize_pairs(&mut self) {
        let vertex_count = self.vertices.len();
        for i in 0..vertex_count {
            for j in (i + 1)..vertex_count {
                let mut pair = Pair::new(i, j);
                pair.compute_error(
                    &self.vertices[i].quadric, 
                    &self.vertices[j].quadric, 
                    &self.vertices[i].position, 
                    &self.vertices[j].position,
                );
                self.heap.push(pair);
            }
        }
    }

    pub fn simplify(&mut self, target_faces: usize) {
        while self.faces.len() > target_faces && !self.heap.is_empty() {
            let pair = self.heap.pop().unwrap();
            let v1 = pair.v1;
            let v2 = pair.v2;

            let new_position = pair.optimal_position;
            let mut new_vertex = Vertex::new(new_position.fixed_rows::<3>(0).into());
            new_vertex.quadric = self.vertices[v1].quadric + self.vertices[v2].quadric;
            
            let new_index = self.vertices.len();
            self.vertices.push(new_vertex);

            self.faces.retain(|face| !face.contains(&v1) && !face.contains(&v2));

            for face in &mut self.faces {
                for vert in face.iter_mut() {
                    if *vert == v1 || *vert == v2 {
                        *vert = new_index;
                    }
                }
            }

            self.heap.clear();
            self.initialize_pairs();
        }
    }

    pub fn save_as_obj(&self, filename: &str) {
        let obj_vertices: Vec<obj_exporter::Vertex> = self.vertices
            .iter()
            .map(|v| obj_exporter::Vertex {
                x: v.position.x,
                y: v.position.y,
                z: v.position.z,
            })
            .collect();

        let obj_shapes: Vec<obj_exporter::Shape> = self.faces
            .iter()
            .map(|face| obj_exporter::Shape {
                primitive: obj_exporter::Primitive::Triangle(
                    (face[0], None, None),
                    (face[1], None, None),
                    (face[2], None, None),
                ),
                groups: vec![],
                smoothing_groups: vec![],
            })
            .collect();

        let set = obj_exporter::ObjSet {
            material_library: None,
            objects: vec![obj_exporter::Object {
                    name: "SimplifiedObject".to_owned(),
                    vertices: obj_vertices,
                    tex_vertices: vec![],
                    normals: vec![],
                    geometry: vec![obj_exporter::Geometry {
                            material_name: None,
                            shapes: obj_shapes,
                        }
                    ]
                }
            ]
        };

        for s in &set.objects[0].geometry[0].shapes {
            println!("{:?}", s);
        }
        obj_exporter::export_to_file(&set, filename)
            .expect("Unable export object output file");
    }

    pub fn result(&self) {
        println!("Simplified vertices : ");
        for (i, v) in self.vertices.iter().enumerate() {
            println!("{}: ({:.3}, {:.3}, {:.3})", i, v.position.x, v.position.y, v.position.z);
        }

        println!("Simplified faces : ");
        for face in &self.faces {
            println!("{} {} {}", face[0], face[1], face[2]);
        }
    }
}
