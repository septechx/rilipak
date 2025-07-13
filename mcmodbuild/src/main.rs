mod binary;
mod cli;
mod installer;
mod structs;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};
use binary::deserialize;
use clap::Parser;
use installer::Installer;
use oxfmt::Serializeable;
use structs::ModBuild;

fn main() -> Result<()> {
    let cli = cli::McModBuild::parse();

    match cli.subcommand {
        cli::Subcommands::Build { file, destination } => build(file, destination)?,
        cli::Subcommands::Install { file, destination } => install(file, destination)?,
    };

    Ok(())
}

fn build(file: String, destination: Option<String>) -> Result<()> {
    let content = fs::read_to_string(file)?;
    let build: ModBuild = serde_yml::from_str(&content)?;

    let id = build.id.clone();
    let name = format!("{}.mcmodbuild", id);
    let path: PathBuf = if let Some(path) = destination {
        resolve_path(&path, &name)
    } else {
        Path::new(&name).to_path_buf()
    };

    fs::write(path, build.serialize()?)?;

    Ok(())
}

fn resolve_path(input: &str, default: &str) -> PathBuf {
    let path = Path::new(input);

    if path.extension().is_some() || path.is_file() {
        path.to_path_buf()
    } else {
        let mut new_path = path.to_path_buf();
        new_path.push(default);
        new_path
    }
}

fn install(file: String, destination: Option<String>) -> Result<()> {
    let content = fs::read_to_string(file)?;
    let build = deserialize(content.as_bytes())?;

    let id = build.id.clone();
    let branch = build.branch.clone();
    let name = format!("{}-{}.jar", id, branch);
    let destination = if let Some(path) = destination {
        resolve_path(&path, &name)
    } else {
        Path::new(&name).to_path_buf()
    };

    let installer = Installer::new(build)?;
    installer.install(destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::structs::{BuildType, ExcludePair, ExcludeType};

    use super::*;

    #[test]
    fn roundtrip_serialize_deserialize() {
        let build = ModBuild {
            id: "testmod".into(),
            name: "Test mod".into(),
            git: "https://repo.git".into(),
            branch: "1.21.7".into(),
            build: BuildType::Cmd,
            cmd: Some("./gradlew build".into()),
            out: "@/target/".into(),
            exclude: vec![
                ExcludePair {
                    type_name: ExcludeType::Ends,
                    value: "-source.jar".into(),
                },
                ExcludePair {
                    type_name: ExcludeType::Starts,
                    value: "dev-".into(),
                },
            ],
        };

        let serialized = serialize(build.clone()).unwrap();
        let deserialized = deserialize(&serialized).unwrap();
        assert_eq!(build, deserialized);
    }
}
