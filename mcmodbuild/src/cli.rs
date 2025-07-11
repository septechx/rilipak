use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct McModBuild {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Build {
        #[clap(required = true)]
        file: String,
    },
    Install {
        #[clap(required = true)]
        file: String,
    },
}
