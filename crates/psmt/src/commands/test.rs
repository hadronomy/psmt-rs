use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[command()]
pub struct TestCommand {
    #[arg(short, long)]
    boolean: bool,
}

impl TestCommand {
    pub fn exec(self) -> Result<(), &'static str> {
        todo!("Execute test command");
    }
}
