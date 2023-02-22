use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

use self::init::InitCommand;
use self::test::TestCommand;

pub mod init;
pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    Test(TestCommand),
    Init(InitCommand),
}
