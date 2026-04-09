use blok::cli::{Cli, Command};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    //TODO: Have a single "run" function that takes the parsed CLI and does the work, so we can test it without invoking the CLI parser.
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
