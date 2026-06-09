use clap::Parser;
use forge::cli::Cli;
use miette::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // println!("{:?}", cli);

    cli.command.run(&cli)
}
