use crate::commands::env::VarType;
use clap::Args;

#[derive(Args, Debug)]
pub struct SetArgs {
    pub name: Option<String>,

    #[arg(long, short)]
    pub description: Option<String>,

    #[arg(long, short = 't')]
    pub var_type: Option<VarType>,

    #[arg(long)]
    pub optional: bool,
}

impl SetArgs {
    pub fn run(&self) -> miette::Result<()> {
        println!("Run forge env set command!");

        println!("{:?}", &self);
        Ok(())
    }
}
