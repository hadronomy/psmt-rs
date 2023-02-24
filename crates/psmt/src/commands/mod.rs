use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

use self::init::InitCommand;
#[cfg(debug_assertions)]
use self::test::TestCommand;

pub mod init;
pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    #[cfg(debug_assertions)]
    Test(TestCommand),
    Init(InitCommand),
}
