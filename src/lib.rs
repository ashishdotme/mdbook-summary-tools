use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

pub mod config;
pub mod discovery;
pub mod model;
pub mod render;
pub mod titles;
pub mod validate;
pub mod writeback;

#[derive(Debug, Parser)]
#[command(name = "mdbook-generate-summary")]
#[command(about = "Generate SUMMARY.md files for mdBook projects")]
pub struct Cli {
    #[arg(long, global = true)]
    pub book: Option<PathBuf>,

    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Generate(GenerateArgs),
    Check(CheckArgs),
    PrintConfig,
}

#[derive(Debug, Clone, Args, Default)]
pub struct GenerateArgs {
    #[arg(long)]
    pub stdout: bool,
}

#[derive(Debug, Clone, Args, Default)]
pub struct CheckArgs {
    #[arg(long)]
    pub diff: bool,
}
