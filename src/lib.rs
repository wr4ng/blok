#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

mod cli;
mod duration;
mod entry;

pub use cli::Cli;
use cli::Command;
use entry::Entry;

pub fn run(cli: Cli) {
    match cli.command {
        Command::Add {
            date,
            duration,
            tags,
            note,
        } => {
            let date = date.unwrap_or_else(|| jiff::Zoned::now().date());
            let entry = Entry::new(date, duration, tags.0, note);
            println!("{entry}");
        }
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
    }
}
