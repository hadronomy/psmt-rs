use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use eyre::Result;
use libpsmt::{ExecutableCommand, ProjectConfig};
use log::info;

/// Initialize a new psmt project
/// in the current directory
#[derive(Parser, Debug)]
#[command()]
pub struct InitCommand {
    #[arg(value_name = "project_dir", default_value_t = String::from("."), required = false)]
    project_directory: String,

    /// Doesn't ask for input and just creates
    /// the default config file
    #[arg(long, short = 'd')]
    default: bool,
}

impl ExecutableCommand for InitCommand {
    fn exec(&self) -> Result<()> {
        // todo!("Not implemented")
        // TODO:
        // Check if psmt is already initialized
        // Show emoji message with an error
        // Ask using inquire crate for the basic info
        // => Design configuration structure
        // TODO: Handle config errors gracefully
        let config_path = Path::new(&self.project_directory).join("psmt.toml");
        info!("Hey");
        if config_path.exists() {
            // TODO: Ask when a file already exists
            eprintln!("You are overriding an already existing psmt project config")
        }
        if self.default {
            let config = ProjectConfig::default().to_string()?;
            let mut config_file = File::create(config_path)?;
            config_file.write_all(config.as_bytes())?;
            return Ok(());
        }
        todo!("WIP: To be implemented");
    }
}
