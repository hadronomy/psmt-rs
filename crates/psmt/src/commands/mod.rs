use clap::Subcommand;
use libpsmt::{executable_cmd, ExecutableCommand};

use self::init::InitCommand;
#[cfg(profile = "dev")]
use self::test::TestCommand;

pub mod init;
pub mod test;

#[derive(Subcommand)]
#[executable_cmd]
pub enum Command {
    #[cfg(profile = "dev")]
    Test(TestCommand),
    Init(InitCommand),
}
