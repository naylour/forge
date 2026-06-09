use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::cli::Cli;

pub mod generate;
pub mod set;

#[derive(Args, Debug)]
pub struct EnvArgs {
    #[arg(long, default_value = "fnox.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: EnvCommand,
}

#[derive(Subcommand, Debug)]
pub enum EnvCommand {
    Set(set::SetArgs),

    Generate(generate::GenerateArgs),
}

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "lower")]
pub enum VarType {
    String,
    Number,
    Bool,
    Url,
    PGUrl,
    Port,
    Uuid,
    Path,
    Json,
}

impl EnvArgs {
    pub fn run(&self, _cli: &Cli) -> miette::Result<()> {
        match &self.command {
            EnvCommand::Generate(cmd) => cmd.run(),
            EnvCommand::Set(cmd) => cmd.run(),
        }
    }
}
