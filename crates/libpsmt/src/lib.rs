use eyre::Result;

pub use libpsmt_proc::*;

pub mod config;
pub mod error;

pub use crate::error::*;
pub use crate::config::*;

pub trait ExecutableCommand {
    fn exec(&self) -> Result<()>;
}
