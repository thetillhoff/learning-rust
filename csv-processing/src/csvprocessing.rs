use std::{collections::HashMap, fs::File};
use csv::Reader;

pub fn process_large_csv(filepath: &str) { //-> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath).expect("Expected file exists");
    let mut rdr = Reader::from_reader(file);

    // Get headers first and store them
    let headers = rdr.headers().unwrap().clone();
    let mut header_map = HashMap::new();

    for (index,header) in headers.iter().enumerate() {
        header_map.insert(header, index);
    }

    let header_index = header_map.get("First Name").expect("header not found");
    
    // Process one record at a time
    for result in rdr.records() {
        let record = result.unwrap();
        // Process your record here
        // Only one record is in memory at a time

        println!("{}",record.get(*header_index).unwrap());
    }
    
    // Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

pub fn test_serde() {
    
}
