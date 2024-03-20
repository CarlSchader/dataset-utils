use clap::Parser;
use std::path::PathBuf;
use std::fs;
use imghdr;

/// Rust program that removes all non-image files from a directory.

#[derive(Parser, Debug)]

#[command(version, about, long_about = None)]
struct Cli {
    /// Input directory
    input_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let mut path_stack: Vec<PathBuf> = Vec::new();
    path_stack.push(cli.input_dir);

    while !path_stack.is_empty() {
        let curr = path_stack.pop().unwrap();
        if curr.is_dir() {
            for entry in fs::read_dir(curr).unwrap() {
                path_stack.push(entry.unwrap().path());
            }
        } else {
            match imghdr::from_file(&curr) {
                Ok(t) => match t {
                    Some(_) => (),
                    None => fs::remove_file(curr).unwrap(),
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
