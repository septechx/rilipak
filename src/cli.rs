use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct RilipakCli {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Build {
        #[clap(short = 'd')]
        destination: Option<PathBuf>,
    },
    Install {
        #[clap(required = true)]
        file: PathBuf,

        #[clap(short = 'd')]
        destination: Option<PathBuf>,
    },
}
