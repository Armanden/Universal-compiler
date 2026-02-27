use std::env;
use std::path::Path;
use std::process::Command;

fn get_extension(file: &str) -> Option<String> {
    Path::new(file)
        .extension()
        .map(|ext| ext.to_string_lossy().to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let file = &args[1];

    match get_extension(file).as_deref() {
        Some("java") => {
            println!("Java file detected");
            Command::new("javac")
                .arg(file)
                .status()
                .expect("Failed to compile Java file");
        }
        Some("c") => {
            println!("C file detected");
            Command::new("gcc")
                .arg(file)
                .arg("-o")
                .arg("program")
                .status()
                .expect("Failed to compile C file");
        }
        Some("cpp") => {
            println!("C++ file detected");
            Command::new("g++")
                .arg(file)
                .arg("-o")
                .arg("program")
                .status()
                .expect("Failed to compile C++ file");
        }
        Some("rs") => {
            println!("Rust file detected");
            Command::new("rustc")
                .arg(file)
                .status()
                .expect("Failed to compile Rust file");
        }
        Some(ext) => println!("Unknown file type: {}", ext),
        None => println!("No extension found"),
    }
}