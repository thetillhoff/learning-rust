use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();
    let source_format = cli::source_format(args.source, args.source_format);
    let destination_format = cli::destination_format(args.destination, args.destination_format);

    println!("source_format: {:?}", source_format);
    println!("destination_format: {:?}", destination_format);

    // read file - batch wise / streaming
    // write file - batch wise / streaming
    // TODO type reflection
}
