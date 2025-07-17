mod check;
mod cli;
mod pack;
mod structs;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;

use oxfmt::Serializable;

use check::assert_valid_config;
use cli::{RilipakCli, Subcommands};
use pack::{read_exclude, zip_dir};
use structs::{Pack, PackConfig, PackMeta};

fn main() -> Result<()> {
    let cli = RilipakCli::parse();

    match cli.subcommand {
        Subcommands::Build { destination } => build(destination),
        //Subcommands::Install { file, destination } => install(file, destination),
        //Subcommands::Init { path } => init(path),
        Subcommands::Check => check(),
        _ => todo!(),
    }
}

fn build(destination: Option<PathBuf>) -> Result<()> {
    let content = fs::read_to_string("pack.yml")?;
    let config: PackConfig = serde_yml::from_str(&content)?;

    assert_valid_config(&config)?;

    let id = config.id.clone();
    let destination = destination.unwrap_or(PathBuf::from(format!("{id}.rilipak")));

    let mut modbuilds: Vec<Box<[u8]>> = Vec::new();
    for file in fs::read_dir("include")? {
        let file = file?;
        let content = fs::read(file.path())?;
        modbuilds.push(content.into_boxed_slice());
    }

    let exclude = read_exclude()?;
    let files = zip_dir(Path::new("./"), &exclude)?;

    let pack: Pack = Pack {
        meta: PackMeta { config, modbuilds },
        include: files.into_boxed_slice(),
    };

    let serialized = pack.serialize()?;

    fs::write(destination, serialized)?;

    Ok(())
}

fn check() -> Result<()> {
    let content = fs::read_to_string("pack.yml")?;
    let config: PackConfig = serde_yml::from_str(&content)?;

    assert_valid_config(&config)
}

//fn install(file: PathBuf, destination: Option<PathBuf>) -> Result<()> {
//    todo!()
//}
//
//fn init(path: PathBuf) -> Result<()> {
//    todo!()
//}
