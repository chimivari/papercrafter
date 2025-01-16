use std::{env, fs, path::Path};

use geometry::{Mesh, Triangle};
use ndarray::Array2;
use wavefront_obj::obj::{self, parse};


mod geometry;

const KNOWN_ARGS_KEYS: &'static[&'static str] = &[
    "-i",           // .obj input path
    "-o",           // .obj output path
    "-m",           // manual path
    "-t",           // pair selection threshold (f64 >= 0) in surface simplification
];

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("No argument found. Please enter 'help'")
    }

    let mut input_path: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut manual_path: Option<String> = None;

    // Surface simplification
    let mut pair_selection_threshold: f64 = 0.;

    // args reading
    // Try to read input_path.obj
    if !KNOWN_ARGS_KEYS.contains(&args[1].as_str()) {
        input_path = Some(args.remove(1));
        // Try to read output_path.obj
        if args.len() > 1 && !KNOWN_ARGS_KEYS.contains(&args[1].as_str()) {
            output_path = Some(args.remove(1));
            // Try to read manual_path.pdf
            if args.len() > 1 && !KNOWN_ARGS_KEYS.contains(&args[1].as_str()) {
                manual_path = Some(args.remove(1));
            }
        }
    }
    // Read keyed args
    while args.len() > 2 {
        let arg_key = &args.remove(1)[..];
        let arg_value = args.remove(1);

        match arg_key {
            "-i" => {
                if input_path.is_some() {
                    panic!("The input path is specified many times");
                }
                else {
                    input_path = Some(arg_value);
                }
            }
            "-o" => {
                if output_path.is_some() {
                    panic!("The output path is specified many times");
                }
                else {
                    output_path = Some(arg_value);
                }
            }
            "-m" => {
                if manual_path.is_some() {
                    panic!("The manual path is specified many times");
                }
                else {
                    manual_path = Some(arg_value);
                }
            }
            "-t" => {
                let threshold = arg_value.parse::<f64>();
                if threshold.is_err() {
                    panic!("-t argument should be followed by a positive floating number. Please enter 'help' command.");
                }
                else {
                    pair_selection_threshold = threshold.unwrap();
                    if pair_selection_threshold < 0. {
                        panic!("-t argument should be followed by a positive floating number. Please enter 'help' command.");
                    }
                }
            }
            _ => panic!("Unknown argument {arg_key}. Please enter the 'help' command.")
        }
    }
    
    if args.len() % 2 != 1 {
        panic!("Invalid number of arguments");
    }

    // Process paper crafting
    paper_crafter(
        input_path.unwrap(), 
        output_path, 
        manual_path, 
        pair_selection_threshold,
    );
}

/// Tranforms the mesh into a pdf with patterns
fn paper_crafter(
        input_path: String, 
        _output_path: Option<String>, 
        _manual_path: Option<String>, 
        _pair_selection_threshold: f64,
    ) {
    let meshes = to_meshes(input_path)
        .expect("Input file parsing error");

    for mesh in meshes {
        // 1. Diminish the number of faces of a mesh
        let mesh = surface_simplification(mesh);
    }
}

/// Parse the input_path.obj to a collection of [`Mesh`]
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

