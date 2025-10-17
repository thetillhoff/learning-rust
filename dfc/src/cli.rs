use clap::Parser;
use anyhow::{Result, bail};
use std::path::Path;

// enum SourceFormat {
//     Csv,
//     Json,
//     Xml,
//     Sqlite,
//     Duckdb,
// }

/// Data format converter
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about, arg_required_else_help = true)]
pub(crate) struct Args {
    /// Source path
    #[arg(short, long, required = false)]
    pub(crate) source: Option<String>,

    /// Source format
    #[arg(long, required = false)]
    pub(crate) source_format: Option<String>,

    /// Destination path
    #[arg(short, long, required = false)]
    pub(crate) destination: Option<String>,

    /// Destination format
    #[arg(long, required = false)]
    pub(crate) destination_format: Option<String>,
}

pub(crate) fn source_format(source: Option<&String>, source_format: Option<&String>) -> Result<String> {
    match (source, source_format) {
        (Some(source), None) => match Path::new(&source).extension() {
            Some(extension) => Ok(extension.to_string_lossy().into_owned()),
            None => {
                bail!("No source file extension nor source format provided");
            }
        },
        (_, Some(source_format)) => Ok(source_format.clone()),
        (None, None) => {
            bail!("No source nor source format provided");
        }
    }
}

pub(crate) fn destination_format(destination: Option<&String>, destination_format: Option<&String>) -> Result<String> {
    match (destination, destination_format) {
        (Some(destination), None) => match Path::new(&destination).extension() {
            Some(extension) => Ok(extension.to_string_lossy().into_owned()),
            None => {
                bail!("No destination file extension nor destination format provided");
            }
        },
        (_, Some(destination_format)) => Ok(destination_format.clone()),
        (None, None) => {
            bail!("No destination or destination format provided");
        }
    }
}
