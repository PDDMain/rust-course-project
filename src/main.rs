use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let directory = args[1].clone();
        println!("{}:", directory);
        let paths = fs::read_dir(directory).unwrap();
        for path in paths {
            println!("|- {}", path.unwrap().path().display());
        };
    } else if args.len() == 4 && args[2] == "--find" {
        let mut vec = Vec::new();
        vec.push(args[1].clone());
        while !vec.is_empty() {
            let directory = vec.remove(0);
            let paths = fs::read_dir(directory).unwrap();
            for path in paths {
                let p = path.unwrap();
                if p.path().is_file() && p.file_name().to_str().unwrap() == args[3] {
                    println!("Full path to the file: {}", p.path().display());
                    return;
                }
                if p.path().is_dir() {
                    vec.push(p.path().display().to_string());
                }
            };
        }
        println!("File `{}` doesn't contains in directory", args[3]);
    }
}
