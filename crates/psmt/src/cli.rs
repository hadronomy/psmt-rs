//! Contains the cli definition

use crate::commands::Command;
use clap::Parser;
use eyre::{Context, Result};
use libpsmt::ExecutableCommand;

// TODO: Override default help

/// Description
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enables debugging
    #[arg(short, long)]
    debug: bool,

    /// The inputted cli subcommand
    #[command(subcommand)]
    command: Command,
}

/// Parses the command line arguments,
/// and executes the matching subcommand
pub fn run() -> Result<Cli> {
    let cli = Cli::parse();
    cli.command.exec().wrap_err("Failed to execute command")?;
    Ok(cli)
}
