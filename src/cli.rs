use crate::commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "forge")]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
