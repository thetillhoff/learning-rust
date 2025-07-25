use clap::Parser;
use std::path::Path;

/// Data format converter
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about, arg_required_else_help = true)]
pub struct Args {
    /// Source path
    #[arg(short, long, required = false)]
    pub source: Option<String>,

    /// Source format
    #[arg(long, required = false)]
    pub source_format: Option<String>,

    /// Destination path
    #[arg(short, long, required = false)]
    pub destination: Option<String>,

    /// Destination format
    #[arg(long, required = false)]
    pub destination_format: Option<String>,
}

pub fn source_format(source: Option<&String>, source_format: Option<&String>) -> String {
    match (source, source_format) {
        (Some(source), None) => match Path::new(&source).extension() {
            Some(extension) => extension.to_string_lossy().into_owned(),
            None => {
                eprintln!("No source file extension nor source format provided");
                std::process::exit(1);
            }
        },
        (_, Some(source_format)) => source_format.clone(),
        (None, None) => {
            eprintln!("No source nor source format provided");
            std::process::exit(1);
        }
    }
}

pub(crate) fn destination_format(destination: Option<&String>, destination_format: Option<&String>) -> String {
    match (destination, destination_format) {
        (Some(destination), None) => match Path::new(&destination).extension() {
            Some(extension) => extension.to_string_lossy().into_owned(),
            None => {
                eprintln!("No destination file extension nor destination format provided");
                std::process::exit(1);
            }
        },
        (_, Some(destination_format)) => destination_format.clone(),
        (None, None) => {
            eprintln!("No destination or destination format provided");
            std::process::exit(1);
        }
    }
}
