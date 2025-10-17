use anyhow::{Result, bail};
use std::fs::File;
use std::io::{BufWriter};
use std::path::Path;

pub(crate) fn open_writer(destination: Option<&String>) -> Result<BufWriter<File>> {
    match destination {
        None => {
            bail!("Could not open destination file")
        }
        Some(destination) => {
            let path = Path::new(destination);
            let file = File::create(&path)?;
            Ok(BufWriter::new(file))
        }
    }
}
