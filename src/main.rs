use core::panic;
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
    if is_obj(file_path) {
        let mut simpl = Simplify::load_obj(file_path, true);
        
        if (simpl.triangles.len() < 3) || (simpl.vertices.len() < 3) {
            panic!("Triangle size or vertices size is less than 3");
        }

        let mut reduce_fraction = reduce_fraction;

        if reduce_fraction > 1.0{
            reduce_fraction = 1.;
        }
        else if reduce_fraction <= 0. {
            panic!("Ratio must be in the range: ]0; 1]");
        }
        let target_count = f32::round((simpl.triangles.len() as f32) * reduce_fraction) as usize;
        
        if target_count < 4 {
            panic!("Object will not survice such extreme decimation");
        }

        let mut start_size = simpl.triangles.len();
        simpl.simplify_mesh(target_count, agressiveness, true);
        if simpl.triangles.len() >= start_size {
            panic!("Unable to reduce mesh");
        }

        if is_obj(export_path) {
            simpl.save_obj(export_path);
            println!("Saved");
        }
    }
    else {
        panic!("File extension not supported : {file_path}");
    }

}

fn is_obj(file_path: &String) -> bool {
    let extension = Path::new(file_path)
        .extension()
        .and_then(OsStr::to_str);
    extension.is_some() && extension.unwrap() == "obj"
}