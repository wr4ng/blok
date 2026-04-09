#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

mod cli;
mod duration;
mod entry;

pub use cli::Cli;
use cli::Command;

pub fn run(cli: Cli) {
    match &cli.command {
        Command::Print {
            tag: _,
            from: _,
            to: _,
        } => todo!("not implemented"),
        Command::Report {
            group_by: _,
            from: _,
            to: _,
            tag: _,
            include_empty: _,
        } => todo!("not implemented"),
        Command::Add {
            date: _,
            duration: _,
            tag: _,
            note: _,
        } => todo!("not implemented"),
    }
}