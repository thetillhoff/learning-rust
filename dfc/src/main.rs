use clap::Parser;
use std::io::{BufRead, Write};

mod cli;
mod reader;
mod writer;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let source_format = cli::source_format(args.source.as_ref(), args.source_format.as_ref())?;
    let destination_format = cli::destination_format(args.destination.as_ref(), args.destination_format.as_ref())?;
    let reader = reader::open_reader(args.source.as_ref())?;

    let mut writer = writer::open_writer(args.destination.as_ref())?;

    match source_format.as_str() {
        "csv" => {
            println!("csv parser");
            for line in reader.lines() {
                let line_content = line?;
                writer.write_all(line_content.as_bytes())?;
                writer.write_all(b"\n")?;
            };
        }
        "json" => {
            println!("json parser");
        }
        _ => {
            println!("unknown format");
        }
    }

    writer.flush()?;

    println!("source_format: {:?}", source_format);
    println!("destination_format: {:?}", destination_format);
    Ok(())
    // read file - batch wise / streaming
    // write file - batch wise / streaming
    // TODO type reflection
}
