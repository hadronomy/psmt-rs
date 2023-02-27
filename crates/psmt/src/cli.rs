//! Contains the cli definition

use crate::commands::Command;
use clap::Parser;
use eyre::{Context, Result};
use libpsmt::ExecutableCommand;

// TODO: Override default help

use owo_colors::OwoColorize;

pub fn _get_help_template() -> String {
    let template = format!(
        "\
{usage_title}

  {arrow} {{usage}}

{about_title}

  {{about}}

{commands_title}

{{subcommands}}

{args_title}

{{options}}
",
        arrow = "❯".bold().green(),
        usage_title = " USAGE         ".bold().on_blue(),
        about_title = " DESCRIPTION   ".bold().on_blue(),
        commands_title = " COMMANDS      ".bold().on_blue(),
        args_title = " ARGUMENTS   ".bold().on_blue(),
    );
    template
}

/// Description
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
// #[command(help_template = get_help_template())]
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
