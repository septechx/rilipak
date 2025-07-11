mod binary;
mod cli;
mod structs;

use std::fs;

use anyhow::{Ok, Result, bail};
use binary::{deserialize, serialize};
use clap::Parser;
use cli::McModBuild;
use structs::{BuildType, ExcludePair, ExcludeType, ModBuild};

fn main() -> Result<()> {
    let cli = McModBuild::parse();

    match cli.subcommand {
        cli::Subcommands::Build { file } => build(file)?,
        cli::Subcommands::Install { file } => install(file)?,
    };

    Ok(())
}

fn build(file: String) -> Result<()> {
    let content = fs::read_to_string(file)?;
    let build: ModBuild = serde_yml::from_str(&content)?;
    let id = build.id.clone();
    let result = serialize(build)?;

    fs::write(format!("{}.mcmodbuild", id), result.as_slice())?;

    Ok(())
}

fn install(file: String) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serialize_deserialize() {
        let build = ModBuild {
            id: "testmod".into(),
            name: "Test mod".into(),
            git: "https://repo.git".into(),
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
