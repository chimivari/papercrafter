use simplify::Simplify;


mod simplify;
mod geometry;
mod math;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    simplify(&args[1], 0.5);
}

fn simplify(filename: &String, _target_ratio: f32) {
    let mut _simplifier = Simplify::load_obj(filename);
}