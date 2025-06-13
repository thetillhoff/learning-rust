mod filepath;
mod fileio;

fn main() {

    let full_filepath = filepath::get_full_filepath("test.txt");

    println!("Reading file {}...", full_filepath);

    let contents = fileio::read_file(&full_filepath);

    println!("Content:\n{}",contents);

}
