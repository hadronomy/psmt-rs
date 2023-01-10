use clap::{Parser, Subcommand};
use crate::cli::Cli;
use crate::commands::Command;

#[derive(Parser)]
#[command()]
pub struct TestCommand {
    #[arg(short, long)]
    boolean: bool,
}

pub fn exec(root: &Cli, cmd: &Command) {
    let command = cm3
}
