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
pub(crate) struct Record {
    pub(crate) inner: Vec<String>
}

pub(crate) struct CsvSource {
    pub(crate) reader: csv::Reader<BufReader<File>>,
}

struct FixedWidthSource {
    spec: Vec<u16>,
    reader: std::io::BufReader<String>,
}

impl Iterator for CsvSource {
    type Item = Record;
    fn next(&mut self) -> Option<Self::Item> {
        let csv_record = self.reader.records().next()?;
        let record = match csv_record {
            Ok(rec) => Record {
                inner: rec.iter().map(|s| s.to_string()).collect()
            },
            Err(_) => return None
        };
        Some(record)
    }
}

impl Iterator for FixedWidthSource {
    type Item = Record;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("");
    }
}

