use std::{env, fs};
use log::error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Expected: 1 argument");
        return
    }
    let directory = args[1].clone();
    println!("{}:", directory);
    let paths = fs::read_dir(directory).unwrap();
    for path in paths {
        println!("|- {}", path.unwrap().path().display());
    }
}
