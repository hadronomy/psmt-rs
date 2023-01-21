//! Contains the cli definition

use crate::commands::Command;
use clap::Parser;
use eyre::{Context, Result};
use libpsmt::ExecutableCommand;

// TODO: Override default help

/// Description
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = false)]
pub struct Cli {
    /// Enables debugging
    #[arg(short, long)]
    pub debug: bool,

    #[command(subcommand)]
    pub commands: Command,
}

/// Parses the command line arguments,
/// and executes the matching subcommand
pub fn run() -> Result<Cli> {
    let cli = Cli::parse();
    cli.commands.exec().wrap_err("Failed to execute command")?;
    Ok(cli)
}
