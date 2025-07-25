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
