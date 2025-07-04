use clap::Parser;
use std::path::Path;

// enum SourceFormat {
//     Csv,
//     Json,
//     Xml,
//     Sqlite,
//     Duckdb,
// }

/// Data format converter
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Source path
    #[arg(short, long, required = false)]
    source: Option<String>,

    /// Source format
    #[arg(long, required = false)]
    source_format: Option<String>,

    /// Destination path
    #[arg(short, long, required = false)]
    destination: Option<String>,

    /// Destination format
    #[arg(long, required = false)]
    destination_format: Option<String>,
}

fn main() {
    let args = Args::parse();

    let source_format = match (args.source, args.source_format) {
        (Some(source), None) => {
            let extension = Path::new(&source).extension();
            match extension {
                Some(extension) => extension.to_str().unwrap().to_string(),
                None => {
                    eprintln!("No file extension nor source format provided");
                    std::process::exit(1);
                }
            }
        },
        (_, Some(source_format)) => source_format,
        (None, None) => {
            eprintln!("No source or source format provided");
            std::process::exit(1);
        }
    };

    let destination_format = match (args.destination, args.destination_format) {
        (Some(destination), None) => {
            let extension = Path::new(&destination).extension();
            match extension {
                Some(extension) => extension.to_str().unwrap().to_string(),
                None => {
                    eprintln!("No file extension nor destination format provided");
                    std::process::exit(1);
                }
            }
        },
        (_, Some(destination_format)) => destination_format,
        (None, None) => {
            eprintln!("No destination or destination format provided");
            std::process::exit(1);
        }
    };

    println!("source_format: {:?}", source_format);
    println!("destination_format: {:?}", destination_format);

    // read file - batch wise / streaming
    // write file - batch wise / streaming
    // TODO type reflection
}
