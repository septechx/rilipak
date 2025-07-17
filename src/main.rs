mod cli;
mod structs;

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;

use cli::Subcommands;

use crate::{cli::RilipakCli, structs::PackConfig};

fn main() -> Result<()> {
    let cli = RilipakCli::parse();

    match cli.subcommand {
        Subcommands::Build { destination } => build(destination),
        Subcommands::Install { file, destination } => install(file, destination),
    }
}

fn build(destination: Option<PathBuf>) -> Result<()> {
    let content = fs::read_to_string("pack.yml")?;
    let config: PackConfig = serde_yml::from_str(&content)?;

    Ok(())
}

fn install(file: PathBuf, destination: Option<PathBuf>) -> Result<()> {
    todo!()
}
