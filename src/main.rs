mod steps;

use anyhow::Result;

fn main() -> Result<()> {
    steps::hk::run()?;
    steps::age::run()?;

    Ok(())
}
