use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(..) => println!("File read."),
        Err(error) => println!("Error: {}", error)
    }
}
