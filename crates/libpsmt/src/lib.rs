pub use libpsmt_impl::*;

pub trait ExecutableCommand {
    fn exec(&self) -> Result<(), &'static str>;
}
