pub use libpsmt_impl::*;

pub trait ExecutableCommand {
    fn exec(&self) -> Result<(), Error>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Unknown Error")]
    Unknown(String),

    #[error("Permission Denied")]
    PermissionDenied,

    #[error("Invalid Argument")]
    InvalidArgument(String),
}
