use std::path::PathBuf;

use clap::Args;

use crate::cli::Cli;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(long, default_value = "~/.config/age/ingot-secret.txt")]
    pub config: PathBuf,
}

impl InitArgs {
    pub fn run(&self, _cli: &Cli) -> miette::Result<()> {
        Ok(())
    }
}
