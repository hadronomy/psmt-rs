use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

use crate::commands::init::InitCommand;
use crate::commands::test::TestCommand;

pub mod init;
pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    /// Test command
    Test(TestCommand),

    /// Initialize a new **psmt project**
    /// in the current directory
    Init(InitCommand),
}
