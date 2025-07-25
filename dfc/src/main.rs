use clap::Parser;
fn main() -> anyhow::Result<()> {
    let args = dfc::cli::Args::parse();
    dfc::run(args)
}
