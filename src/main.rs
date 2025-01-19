use std::{env, fs, path::Path};

use mesh_simplifier::{Mesh, Vertex};
use nalgebra::Vector3;
use wavefront_obj::obj::{self, parse};


mod geometry;
mod mesh_simplifier;

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
        output_path: Option<String>, 
        manual_path: Option<String>, 
        pair_selection_threshold: f64,
    ) {
    let mut meshes = to_meshes(input_path.clone())
        .expect("Input file parsing error");

    let output_path = if output_path.is_some() {
        output_path.unwrap()
    } else {
        "patterns_".to_owned() + &input_path.clone()
    };

    let manual_path = if manual_path.is_some() {
        manual_path.unwrap()
    } else {
        "manual_".to_owned() + &input_path
    };

    for mesh in &mut meshes {
        println!("start -> {}", mesh.vertices.len());
        mesh.simplify(0.5);
        println!("end -> {}", mesh.vertices.len());
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
        let vertices: Vec<Vertex> = object.vertices
            .iter()
            .map(|v| Vertex::new(Vector3::new(v.x, v.y, v.z)))
            .collect();

        let mut faces = Vec::new();
        for g in object.geometry {
            for s in g.shapes {
                match s.primitive {
                    obj::Primitive::Triangle(v1, v2, v3) => {
                        faces.push((v1.0, v2.0, v3.0));
                    }
                    _ => ()
                };
            }
        }
        meshes.push(Mesh::new(vertices, faces, 0.));
    }

    Ok(meshes)
}