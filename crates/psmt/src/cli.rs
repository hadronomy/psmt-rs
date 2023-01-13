use crate::commands::Command;
use clap::Parser;

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
pub fn run() -> Result<Cli, &'static str> {
    let cli = Cli::parse();
    match cli.commands.exec(&cli) {
        Ok(_) => Ok(cli),
        Err(msg) => Err(msg),
    }
}
