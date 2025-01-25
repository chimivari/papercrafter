use simplify::Simplify;


mod simplify;
mod geometry;
mod math;

const KNOWN_ARGS_KEYS: &'static[&'static str] = &[
    "-i",           // .obj input path
    "-fr",           // face ration ]0; 1]
];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        panic!("No argument found. Please enter -help or -h command");
    }

    let mut input_path: Option<&str> = None;
    let mut face_ratio: f32 = 0.8;

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
            "-fr" => {
                if let Some(ratio) = args.get(arg_i + 1) {
                    face_ratio = ratio.parse::<f32>()
                        .expect("Cannot convert the face ratio");
                } 
                else {panic!("Please enter a float number between 0. and 1. after the -fr argument")}
                arg_i += 1;
            }
            _ => panic!("Unknown arg entered")
        }
        arg_i += 1;
    }
    
    simplify(
        input_path
        .expect("Input file expected. Please enter '-i input_file.obj'"), 
        face_ratio
    );
}

fn simplify(filename: &str, target_ratio: f32) {
    let mut target_ratio = target_ratio;
    let mut simplifier = Simplify::load_obj(filename);

    if target_ratio > 1. {target_ratio = 1.}
    else if target_ratio <= 0. {panic!("Face ratio must be in the range : ]0; 1]")}

    let target_count = f32::round((simplifier.nb_triangles() as f32) * target_ratio) as usize;
    
}