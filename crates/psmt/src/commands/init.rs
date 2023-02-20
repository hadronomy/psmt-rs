use std::fs::File;
use std::io::Write;

use clap::Parser;
use eyre::Result;
use libpsmt::ProjectConfig;

#[derive(Parser, Debug)]
#[command()]
pub struct InitCommand {}

impl InitCommand {
    pub fn exec(&self) -> Result<()> {
        // todo!("Not implemented")
        // TODO:
        // Check if psmt is already initialized
        // Show emoji message saying with an error
        // Ask using inquire crate for the basic info
        // => Design configuration structure
        let config = ProjectConfig::get_default().serialize()?;
        let mut config_file = File::create("psmt.toml")?;
        config_file.write_all(config.as_bytes())?;
        Ok(())
    }
}
