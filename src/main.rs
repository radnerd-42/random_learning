use std::io::{Read};
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error reading file path.");
        return;
    }
    let file_path = &args[1];
    println!("Reading {}...", file_path);
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}.", err);
            return;
        }
    };

    let mut contents = String::new();
    let word_count: Vec<&str> = contents.split_whitespace().collect();
    
    println!("The file contains {} words.", word_count.len());

//    match file.read_to_string(&mut contents) {
//        Ok(_) => println!("File contents: {}", contents),
//        Err(err) => println!("Failed to read file: {}", err),
//    };
}
