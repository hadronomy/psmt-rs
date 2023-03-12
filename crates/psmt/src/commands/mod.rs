use clap::Subcommand;
use libpsmt::ExecutableCommand;

use self::init::InitCommand;
#[cfg(debug_assertions)]
use self::test::TestCommand;

pub mod init;
pub mod test;

#[derive(Subcommand, ExecutableCommand)]
pub enum Command {
    #[cfg(debug_assertions)]
    Test(TestCommand),
    Init(InitCommand),
}
