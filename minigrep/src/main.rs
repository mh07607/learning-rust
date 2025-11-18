use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for '{}' in file '{}'", query, filename);

    let contents: String = fs::read_to_string(filename)
        .expect(&format!("Not able to read file '{}'", filename));

    println!("File Content: \n{contents}");
}
