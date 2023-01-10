use crate::commands::test::TestCommand;
use clap::{Subcommand};
use crate::cli::Cli;

pub mod test;

#[derive(Subcommand)]
pub enum Command {
    /// Test command
    Test(TestCommand),
}

// TODO: Macro that does this automatically :)
pub fn builtin_exec(cmd: &str) -> Option<fn(&Cli, &Command)> {
    let exec = match cmd {
        "test" => test::exec,
        _ => return None,
    };
    Some(exec)
}
