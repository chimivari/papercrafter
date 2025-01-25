use std::{fs::{self, File}, io::{self, BufRead}, iter::Map};

use wavefront_obj::obj::{self, parse};

use crate::{geometry::{reference::Ref, triangle::Triangle, vertex::Vertex}, math::{vec2f::Vec2F, vec3f::Vec3F}};



pub struct Simplify {
    triangles: Vec<Triangle>,
    vertices: Vec<Vertex>,
    refs: Vec<Ref>,
}

impl Simplify {
    /// Load [.obj] file
    pub fn load_obj(filename: &str) -> Self {
        let content = fs::read_to_string(filename)
            .expect(format!("Cannot read {filename}").as_str());
        let objset = parse(content)
            .expect(format!("Parsing error of {filename}").as_str());
        let mut nvertices = 0;
        let mut ntriangles = 0;

        // Count the number of triangles and vertices
        for object in &objset.objects {
            nvertices += object.vertices.len();
            for geo in &object.geometry {
                ntriangles += geo.shapes.len();
            }
        }

        let mut this = Simplify {
            vertices: Vec::with_capacity(nvertices),
            triangles: Vec::with_capacity(ntriangles),
            refs: Vec::new(),
        };

        for object in &objset.objects {
            // Convert all obj::Vertex to Vertex
            this.to_vertices(&object.vertices);
            // Convert all obj::Primitive::Triangle to Triangle
            for geo in &object.geometry {
                this.to_triangles(&geo.shapes);
            }
        }

        this
    }

    /// Convert a vector of [obj::Vertex] into a vector of [Vertex]</br>
    /// Add them to self.vertices
    fn to_vertices(&mut self, obj_vertices: &Vec<obj::Vertex>) {
        for v in obj_vertices {
            self.vertices.push(
                Vertex::new(
                    Vec3F::new(v.x, v.y, v.z)
                )
            );
        }
    }

    fn to_triangles(&mut self, obj_triangles: &Vec<obj::Shape>) {
        use obj::Primitive;
        for t in obj_triangles {
            match t.primitive {
                Primitive::Triangle(vint1, vint2, vint3) => {
                    self.triangles.push(
                        Triangle::new(vint1.0, vint2.0, vint3.0)
                    );
                }
                _ => (panic!("Cannot add a line or a point to triangles"))
            }
        }
    }

    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }
}