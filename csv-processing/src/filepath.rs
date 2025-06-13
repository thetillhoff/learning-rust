use std::{env, path::Path};

pub fn get_full_filepath(relative_path: &str) -> String {
    let current_dir = env::current_dir().expect("An error occured");
    println!("The current directory is {}", current_dir.display());

    let file_path = Path::new(&current_dir.to_str().unwrap()).join(relative_path);
    return file_path.to_str().unwrap().to_string();
}
