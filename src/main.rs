use std::fs;

use geometry::{triangle::Triangle, vertex::Vertex};
use math::vec3f::Vec3F;
use simplify::Simplify;
use wavefront_obj::obj;


mod simplify;
mod geometry;
mod math;

const KNOWN_ARGS_KEYS: &'static[&'static str] = &[
    "-i",           // .obj input path
    "tr",           // target reduction ]0; 1]
    "tc",           // target count ]0; +inf[
];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        panic!("No argument found. Please enter -help or -h command");
    }

    let mut input_path: Option<&str> = None;
    let mut target_reduction: Option<f64> = None;
    let mut target_count: Option<usize> = None;
    

    let mut arg_i = 1;

    if !KNOWN_ARGS_KEYS.contains(&args[1].as_str()) {
        input_path = Some(args[1].as_str());
        arg_i += 1;
    }

    while arg_i < args.len() {
        let arg = args[arg_i].as_str();
        match arg {
            "-i" => {
                if let Some(path) = args.get(arg_i + 1) {
                    input_path = Some(path);
                }
                else {panic!("Please enter a .obj file path after the -i argument")}
                arg_i += 1;
            }
            "-tr" => {
                if let Some(ratio) = args.get(arg_i + 1) {
                    target_reduction = Some(ratio.parse::<f64>()
                    .expect("Cannot prase the target reduction"));
                } 
                else {panic!("Please enter a float number between 0. and 1. after the -tr argument")}
                arg_i += 1;
            }
            "-tc" => {
                if let Some(count) = args.get(arg_i + 1) {
                    target_count = Some(count.parse::<usize>()
                    .expect("Cannot prase the target count"));
                }
                else {panic!("Please enter an integer bigger than 1 after the -tc argument")}
                arg_i += 1;
            }

            _ => panic!("Unknown arg entered : {}", arg)
        }
        arg_i += 1;
    }

    
    let (vertices, triangles) = to_mesh(input_path.expect(
        "Please enter an input path with '-i input_path.obj'"
    ));

    // Check args
    if (target_reduction.is_none() && target_count.is_none()) || (target_reduction.is_some() && target_count.is_some()) {
        panic!("Please enter a value for -tr and -tc, but not both");
    }
    else if target_reduction.is_some() {
        if target_reduction.unwrap() > 1. {target_reduction = Some(1.)}
        else if target_reduction.unwrap() <= 0. {panic!("Please enter a float number bigger than 0 for -tr argument")}
        target_count = Some((target_reduction.unwrap() * triangles.len() as f64) as usize);
    }
    else if target_count.is_some() {
        if target_count.unwrap() == 0 {
            panic!("Please enter an integer bigger than 0 for -tc number");
        }
    }

    simplify(
        vertices,
        triangles,
        target_count.unwrap(),
        7,
        false,
        false,
    )
}

fn to_mesh(filename: &str) -> (Vec<Vertex>, Vec<Triangle>) {
    use obj::Primitive;
    let content = fs::read_to_string(filename)
        .expect(format!("Cannot read {filename}").as_str());
    let objset = obj::parse(content)
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

    let mut vertices = Vec::with_capacity(nvertices);
    let mut triangles = Vec::with_capacity(ntriangles);

    for object in &objset.objects {
        // Convert all obj::Vertex to Vertex
        for v in &object.vertices {
            vertices.push(
                Vertex::new(
                    Vec3F::new(v.x, v.y, v.z)
                )
            );
        }
        // Convert all obj::Primitive::Triangle to Triangle
        for geo in &object.geometry {
            for t in &geo.shapes {
                match t.primitive {
                    Primitive::Triangle(vint1, vint2, vint3) => {
                        triangles.push(
                            Triangle::new(vint1.0, vint2.0, vint3.0)
                        );
                    }
                    _ => (panic!("Cannot add a line or a point to triangles"))
                }
            }
        }
    }

    (vertices, triangles)
}

fn simplify(
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    target_count: usize,
    agg: usize,
    verbose: bool,
    return_collapses: bool,
) {
    let n_faces = triangles.len();
}