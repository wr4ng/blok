use blok::cli::{Cli, Command};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    //TODO: Have a single "run" function that takes the parsed CLI and does the work, so we can test it without invoking the CLI parser.
    match &cli.command {
        Command::Print { tag, from, to } => todo!("not implemented"),
        Command::Report {
            group_by,
            from,
            to,
            tag,
            include_empty,
        } => todo!("not implemented"),
        Command::Add {
            date,
            duration,
            tag,
            note,
        } => todo!("not implemented"),
    }
}
