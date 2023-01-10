use clap::{Parser, Subcommand};
use enumflags2::{bitflags, make_bitflags, BitFlag, BitFlags};
use human_panic::setup_panic;

mod cli;
mod commands;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum MessageOption {
    RespectSeverity,
    Colorize,
}

fn main() {
    setup_panic!();
    cli::run();
    let options: BitFlags<MessageOption> = MessageOption::RespectSeverity | MessageOption::Colorize;
    if options.contains(MessageOption::Colorize) {
        println!("This are the options: {}", options);
    }
}
