use human_panic::setup_panic;
use colored::Colorize;

mod cli;
mod commands;

fn main() {
    setup_panic!();
    if let Err(error) = cli::run() {
        println!("{} {}",
            format!("❌"),
            format!("{}", error).red().bold()
        );
    }
}
