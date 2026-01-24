use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Sirt {
    #[command(subcommand)]
    pub command: SirtCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SirtCommand {
    From {
        input: String,
        #[arg(long, short)]
        using: Using,
    },
    File {
        path: PathBuf,
        using: Using,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Using {
    Gui,
    Tui,
}
