use anyhow::{Result, bail};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub(crate) fn open_reader(source: Option<&String>) -> Result<BufReader<File>> {
    match source {
        None => {
            bail!("Could not open source file")
        }
        Some(source) => {
            let path = Path::new(source);
            let file = File::open(&path)?;
            Ok(BufReader::new(file))
        }
    }
}

// Maybe could be helpful later if we want to define some methods like filter, map, dropColumn, etc.
struct Record {
    inner: Vec<String>
}

struct CsvSource {
    delimeter: String,
    reader: csv::Reader<std::io::BufReader<String>>,
}

struct FixedWidthSource {
    spec: Vec<u16>,
    reader: std::io::BufReader<String>,
}

impl Iterator for CsvSource {
    type Item = Vec<String>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("");
    }
}

impl Iterator for FixedWidthSource {
    type Item = Vec<String>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("");
    }
}

