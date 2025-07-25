use std::io::BufRead;

pub mod cli;
// pub mod reader;

pub fn run(args: cli::Args) -> anyhow::Result<()> {
    let source_format = cli::source_format(args.source.as_ref(), args.source_format.as_ref());
    let destination_format = cli::destination_format(args.destination.as_ref(), args.destination_format.as_ref());

    libreaders::csv::open_reader(args.source.as_ref())?;
    // let reader = reader::open_reader(args.source.as_ref())?;
    // for line in reader.lines() {
    //     println!("{}", line.unwrap())
    // }
    println!("source_format: {:?}", source_format);
    println!("destination_format: {:?}", destination_format);
    Ok(())
    // read file - batch wise / streaming
    // write file - batch wise / streaming
    // TODO type reflection
}
