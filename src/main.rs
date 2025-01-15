use std::env;


mod geometry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("No argument found. Please enter 'help'")
    }
}
