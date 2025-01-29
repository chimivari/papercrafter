

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        show_help(&args);
        return;
    }
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