use clap::Args;

#[derive(Args, Debug)]
pub struct GenerateArgs {}

impl GenerateArgs {
    pub fn run(&self) -> miette::Result<()> {
        println!("Run forge env generate command!");
        Ok(())
    }
}
