#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

mod cli;
mod duration;
mod entry;

use std::fs::OpenOptions;
use std::io::Write;

pub use cli::Cli;
use cli::Command;
use entry::Entry;

pub fn run(cli: Cli) {
    if !cli.file.exists() {
        eprintln!("blok file '{}' does not exist", cli.file.display());
        return;
    }

    match cli.command {
        Command::Add {
            date,
            duration,
            tags,
            note,
        } => {
            let date = date.unwrap_or_else(|| jiff::Zoned::now().date());
            let entry = Entry::new(date, duration, tags.0, note);

            let mut file = match OpenOptions::new().append(true).open(&cli.file) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("failed to open '{}': {}", cli.file.display(), e.kind());
                    return;
                }
            };
            if let Err(e) = writeln!(file, "{entry}") {
                eprintln!(
                    "failed to write new entry to '{}': {}",
                    cli.file.display(),
                    e.kind()
                );
                return;
            }
            println!("Entry added:\n{entry}");
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
