use human_panic::setup_panic;

mod cli;
mod commands;

fn main() {
    setup_panic!();
    match cli::run() {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

