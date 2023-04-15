use anyhow::Result;
use clap::Parser;
use rxo::{run, Cli};

fn main() -> Result<()> {
    let mut args = Cli::parse();

    run(&mut args)?;

    Ok(())
}
