use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];
    let file_path = &args[2];

    println!("searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Not able to read the file");
    println!("{contents}");
}
