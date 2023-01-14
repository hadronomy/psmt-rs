use crate::commands::test::TestCommand;
use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    /// Test command
    Test(TestCommand),
}

// TODO: Macro that does this automatically :)
