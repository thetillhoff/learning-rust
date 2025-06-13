use std::{env, path::Path};
use std::fs;


fn main() {

    let current_dir = env::current_dir().expect("An error occured");
    println!("The current directory is {}", current_dir.display());

    let file_path = Path::new(&current_dir.to_str().unwrap()).join("test.txt");

    println!("Reading file {}...", file_path.to_str().unwrap());

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
