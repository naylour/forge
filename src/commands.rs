use clap::Subcommand;

use crate::cli::Cli;

pub mod env;
pub mod init;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Env(env::EnvArgs),

    Init(init::InitArgs),
}

impl Commands {
    pub fn run(&self, cli: &Cli) -> miette::Result<()> {
        match self {
            Commands::Env(cmd) => cmd.run(cli),
            Commands::Init(cmd) => cmd.run(cli),
        }
    }
}
