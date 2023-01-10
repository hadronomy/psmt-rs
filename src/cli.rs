use crate::commands::{Command};
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

pub fn run() {
    let cli = Cli::parse();
}
