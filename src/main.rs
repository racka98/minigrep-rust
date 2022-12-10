use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}\n", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should be able to read the file!");

    println!("With text contents:\n{}", contents);
}
