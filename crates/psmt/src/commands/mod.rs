use crate::cli::Cli;
use crate::commands::test::TestCommand;
use clap::Subcommand;

pub mod test;

#[derive(Subcommand)]
pub enum Command {
    /// Test command
    Test(TestCommand),
}

impl Command {
    pub fn exec(&self, root: &Cli) -> Result<(), &'static str> {
        match &self {
            Command::Test(cmd) => cmd.exec(root),
        }
    }
}
// TODO: Macro that does this automatically :)
