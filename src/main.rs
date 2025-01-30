use std::{ffi::OsStr, path::Path};

use simplify::Simplify;

mod simplify;
mod geometry;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        show_help(&args);
        return;
    }

    let file_path = args[1].clone();
    let export_path = args[2].clone();
    let mut reduce_fraction = 0.5;
    if args.len() > 3 {
        reduce_fraction = args[3].clone().parse::<f32>()
            .expect("Ratio cannot be parsed");
    }

    let mut agressiveness = 7.0;
    if args.len() > 4 {
        agressiveness = args[4].parse::<f32>()
            .expect("Agressiveness cannot be parsed");
    }

    simplify(&file_path, &export_path, reduce_fraction, agressiveness);
}

fn show_help(args: &Vec<String>) {
    println!("Usage: {} <input.obj> <output.obj> <ratio> <agressiveness>", args[0]);
    println!("<input> :  Name of existing .obj file");
    println!("<output> :  Name for decimated .obj file");
    println!("<ratio> : (default 0.5) for example 0.2 will decimate 80% of triangles");
    println!("<agressiveness> : (default 7.0) faster or better decimation");
    println!("Example : ");
    println!("  {} monkey.obj decimated_monkey.obj 0.2", args[0]);
}

fn simplify(file_path: &String, export_path: &String, reduce_fraction: f32, agressiveness: f32) {
    let simpl: Simplify;
    if is_obj(file_path) {
        simpl = Simplify::load_obj(file_path, true);
        for i in &simpl.materials {
            println!("{i}");
        }
        println!("\n{:?}\n", simpl.mtllib);
        for i in &simpl.vertices {
            println!("{:?}", i);
        }
        println!("\n");
        for i in &simpl.triangles {
            println!("{:?}", i);
        }
    }
    else {
        println!("File extension not supported : {file_path}");
        return;
    }
}

fn is_obj(file_path: &String) -> bool {
    let extension = Path::new(file_path)
        .extension()
        .and_then(OsStr::to_str);
    extension.is_some() && extension.unwrap() == "obj"
}