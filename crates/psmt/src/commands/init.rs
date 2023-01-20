use eyre::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct InitCommand {}

impl InitCommand {
    pub fn exec(&self) -> Result<()> {
        todo!("Not implemented")
        // TODO:
        // Check if psmt is already initialized
        // Show emoji message saying with an error
        // Ask using inquire crate for the basic info
        // => Design configuration structure
    }
}
