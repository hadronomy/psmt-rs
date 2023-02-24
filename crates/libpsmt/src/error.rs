use config::ConfigError;
use std::process::{ExitCode, Termination};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown Error: `{0}`")]
    Unknown(String),

    #[error("Internal Error: `{0}`")]
    Internal(String),

    #[error("Internal Git Error: `{0}`")]
    Git(String),

    #[error("Permission Denied")]
    PermissionDenied,

    #[error("Invalid Argument: `{0}`")]
    InvalidArgument(String),

    #[error("Invalid Config: `{0}`")]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Other(#[from] eyre::Error),
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        match self {
            Self::Unknown(_) => ExitCode::from(1),
            _ => ExitCode::from(255),
        }
    }
}
