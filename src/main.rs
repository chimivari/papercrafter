use std::{collections::HashMap, env, fs, path::Path};

use geometry::{Mesh, Triangle};
use ndarray::{arr2, Array2};
use wavefront_obj::obj::{self, parse};


mod geometry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("No argument found. Please enter 'help'")
    }

    let known_args: [String; 1] = [
        "-m".to_string(),                   // .obj path
    ];

    // Check the first arg
    if !known_args.contains(&args[1]) {
        let meshes = to_meshes(Path::new(&args[1]))
            .expect(format!("The file '{}' coundn't be red and parsed", &args[1]).as_str());
        paper_crafter(meshes, String::new());
    }
}

fn to_meshes<S>(path: S) -> Result<Vec<Mesh>, Box<dyn std::error::Error>> 
where 
    S: AsRef<Path>
{
    let content = fs::read_to_string(path)?;
    let objects = parse(content)?.objects;
    let mut meshes: Vec<Mesh> = Vec::new();

    for object in objects {
        let vertices = geometry::to_vertices(object.vertices);
        let mut triangles = Vec::new();
        for g in object.geometry {
            for s in g.shapes {
                match s.primitive {
                    obj::Primitive::Triangle(v1, v2, v3) => {
                        let (iv1, iv2, iv3) = (v1.0, v2.0, v3.0);
                        let triangle = Triangle::new(
                            vertices[iv1].clone(), 
                            vertices[iv2].clone(), 
                            vertices[iv3].clone(),
                        );

                        triangles.push(triangle);
                    }
                    _ => ()
                };
            }
            meshes.push(Mesh::new(triangles.clone(), vertices.clone()));
        }
    }

    Ok(meshes)
}

/// Tranforms the mesh into a pdf with patterns
fn paper_crafter(meshes: Vec<Mesh>, output_path: String) {
    for mesh in meshes {
        // 1. Diminish the number of faces of a mesh
        let mesh = surface_simplification(mesh);
    }
}

/// Surface simplification using quadric error metrics (by M. Garland & P.S. Heckbert)</br>
/// Diminish the number of faces of a mesh
fn surface_simplification(mesh: Mesh) -> Mesh {
    for v in &mesh.vertices {
        // Compute the Q matrices for all initial vertices
        let mut q: Array2<f64> = Array2::<f64>::zeros((4, 4));
        for t in &mesh.triangles {
            if t.contains_point(&v) {
                q += &t.fund_err_quad;
            }
        }
    }
    mesh
}

