mod filepath;
// mod fileio;
// mod csvprocessing;
mod csvprocessingserde;

fn main() {

    let full_filepath = filepath::get_full_filepath("customers.csv");
    
    // println!("Reading file {}...", full_filepath);
    // let contents = fileio::read_file(&full_filepath);
    // println!("Content:\n{}",contents);

    csvprocessingserde::process_large_csv(&full_filepath);

}
