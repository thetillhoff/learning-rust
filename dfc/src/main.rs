use clap::Parser;

mod cli;
mod reader;
mod writer;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let source_format = cli::source_format(args.source.as_ref(), args.source_format.as_ref())?;
    let destination_format = cli::destination_format(args.destination.as_ref(), args.destination_format.as_ref())?;

    println!("source_format: {:?}", source_format);
    println!("destination_format: {:?}", destination_format);

    let reader = reader::open_reader(args.source.as_ref())?;
    let mut writer = writer::open_writer(args.destination.as_ref())?;

    match source_format.as_str() {
        "csv" => {

            let mut csv_writer = csv::WriterBuilder::new()
                .delimiter(b';') // TODO reuse same delimiter and make it configurable
                .from_writer(&mut writer);
            // TODO Note that the CSV writer is buffered automatically, so you should not wrap wtr in a buffered writer like io::BufWriter.
            //      https://docs.rs/csv/latest/csv/struct.Writer.html

            let csv_reader = csv::ReaderBuilder::new()
                .delimiter(b',') // TODO get delimiter from file and make it configurable
                .from_reader(reader);

            let csv_source = reader::CsvSource {
                reader: csv_reader,
            };

            // // TODO make this configurable
            // let headers = csv_reader.headers()?;
            // csv_writer.write_record(headers)?;

            for record in csv_source {
                csv_writer.write_record(&record.inner)?;
            };

            csv_writer.flush()?;
        }
        "json" => {
            println!("json parser");
        }
        _ => {
            println!("unknown format");
        }
    }

    Ok(())
    // read file - batch wise / streaming
    // write file - batch wise / streaming
    // TODO type reflection
}
