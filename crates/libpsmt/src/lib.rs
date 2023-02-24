use eyre::Result;

pub use libpsmt_proc::*;

pub mod config;
pub mod error;

pub use crate::config::*;
pub use crate::error::*;

pub trait ExecutableCommand {
    fn exec(&self) -> Result<()>;
}
