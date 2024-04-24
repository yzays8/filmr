use anyhow::Result;
use clap::Parser;

use crate::cli::Args;

pub fn run() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
