use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

use crate::commands::test::TestCommand;

pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    /// Test command
    Test(TestCommand),
}
