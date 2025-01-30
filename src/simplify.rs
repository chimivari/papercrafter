use std::{collections::HashMap, fs::{self, File}};
use std::io::Write;

use nalgebra::Vector3;
use wavefront_obj::obj;

use crate::geometry::{Triangle::Triangle, Vertex::Vertex};


pub struct Simplify {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub mtllib: Option<String>,
    pub materials: Vec<String>,
}

impl Simplify {
    pub fn load_obj(file_path: &String, process_uv: bool) -> Self {
        let content = fs::read_to_string(file_path)
            .expect(format!("Cannot read {file_path}").as_str());
        let objset = obj::parse(content)
            .expect(format!("Cannot parse {file_path}").as_str());
        let mtllib = objset.material_library;
        
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut materials = Vec::new();
        let mut uvs = Vec::new();
        let mut uv_map = Vec::new();
        let mut material_map = HashMap::new();
        let mut material: i32 = -1;

        for object in &objset.objects {
            for v in &object.vertices {
                vertices.push(Vertex::new(v.x, v.y, v.z));
            }

            for t in &object.tex_vertices {
                uvs.push(Vector3::new(t.u, t.v, t.w));
            }

            for geo in &object.geometry {
                if let Some(usemtl) = &geo.material_name {
                    if !material_map.contains_key(&usemtl) {
                        material_map.insert(usemtl, materials.len() as i32);
                        materials.push(usemtl.clone());
                    }
                    material = material_map[usemtl];
                }
                
                for s in &geo.shapes {
                    match s.primitive {
                        obj::Primitive::Triangle(v1, v2, v3) => {
                            let mut triangle = Triangle {
                                v: [v1.0, v2.0, v3.0],
                                attr: 0,
                                material: 0,
                                uvs: [Vector3::zeros(); 3],
                                deleted: false,
                            };
                            if process_uv && 
                                v1.1.is_some() &&
                                v2.1.is_some() && 
                                v3.1.is_some() 
                            {
                                let indices = vec![
                                    v1.1.unwrap(),
                                    v2.1.unwrap(),
                                    v3.1.unwrap(),
                                ];
                                uv_map.push(indices);
                                // triangle.attr |= Attributes.TexCoord
                                triangle.attr |= 4;
                            }
                            triangle.material = material;
                            triangles.push(triangle);
                        }
                        _ => ()
                    }
                }
            }
        }

        if process_uv && uvs.len() > 0 {
            for i in 0..triangles.len() {
                for j in 0..3 {
                    triangles[i].uvs[j] = uvs[uv_map[i][j]];
                }
            }
        }

        Self {
            vertices,
            triangles,
            mtllib,
            materials,
        }
    }

    pub fn simplify_mesh(&mut self, target_count: usize, agressiveness: f32, verbose: bool) {

    }

    pub fn save_obj(&self, export_path: &String) {
        let mut file = File::create(export_path)
            .expect(format!("Cannot write output file {export_path}").as_str());

        let mut crt_material = -1;
        let has_uv = (self.triangles.len() > 0 ) &&
            (self.triangles[0].attr & 4) == 4;  // 4 = TexCoord
        
        if let Some(mtllib) = &self.mtllib {
            writeln!(file, "mtllib {mtllib}")
                .expect("Cannot wrote mtllib");
        }
        for i in 0..self.vertices.len() {
            writeln!(file, "v {} {} {}", 
                self.vertices[i].p.x, 
                self.vertices[i].p.y,
                self.vertices[i].p.z,
            )
            .expect("Cannot write vertices");
        }
        if has_uv {
            for i in 0..self.triangles.len() {
                if !self.triangles[i].deleted {
                    writeln!(file, "vt {} {}",
                        self.triangles[i].uvs[0].x,
                        self.triangles[i].uvs[0].y,
                    ).expect("Cannot write vertex textures");
                    writeln!(file, "vt {} {}",
                        self.triangles[i].uvs[1].x,
                        self.triangles[i].uvs[1].y,
                    ).expect("Cannot write vertex textures");
                    writeln!(file, "vt {} {}",
                        self.triangles[i].uvs[2].x,
                        self.triangles[i].uvs[2].y,
                    ).expect("Cannot write vertex textures");
                }
            }
        }
        let mut uv = 1;
        for i in 0..self.triangles.len() {
            if !self.triangles[i].deleted {
                if self.triangles[i].material != crt_material {
                    crt_material = self.triangles[i].material;
                    writeln!(file, "usemtl {}", 
                        self.materials[self.triangles[i].material as usize]
                    ).expect("Cannot write usemtl");
                }
                if has_uv {
                    writeln!(file, "f {}/{} {}/{} {}/{}",
                        self.triangles[i].v[0] + 1,
                        uv,
                        self.triangles[i].v[1] + 1,
                        uv + 1,
                        self.triangles[i].v[2] + 1,
                        uv + 2,
                    ).expect("Cannot write face");
                    uv += 3;
                }
                else {
                    writeln!(file, "f {} {} {}",
                        self.triangles[i].v[0]+1,
                        self.triangles[i].v[1]+1,
                        self.triangles[i].v[2]+1,
                    ).expect("Cannot write face");
                }
            }
        }
    }
}

#[allow(dead_code)]
enum Attributes {
    None,
    Normal = 2,
    TexCoord = 4,
    Color = 8,
}

