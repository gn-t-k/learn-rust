use std::env::args;
use std::fs::read_to_string;

fn run_cat(path: String) {
    match read_to_string(path) {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    }
}

fn main() {
    match args().nth(1) {
        Some(path) => run_cat(path),
        None => println!("No path is specified!"),
    }
}
