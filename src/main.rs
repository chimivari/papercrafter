use std::{env, fs, path::Path};

use geometry::{Mesh, Triangle};
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
        let _meshes = to_meshes(Path::new(&args[1]))
            .expect(format!("The file '{}' coundn't be red and parsed", &args[1]).as_str());
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
        for g in object.geometry {
            let mut mesh = Mesh::new();
            for s in g.shapes {
                match s.primitive {
                    obj::Primitive::Triangle(v1, v2, v3) => {
                        let (iv1, iv2, iv3) = (v1.0, v2.0, v3.0);
                        let triangle = Triangle(vertices[iv1].clone(), vertices[iv2].clone(), vertices[iv3].clone());
                        mesh.triangles.push(triangle);
                    }
                    _ => ()
                };
            }
            meshes.push(mesh);
        }
    }

    Ok(meshes)
}
