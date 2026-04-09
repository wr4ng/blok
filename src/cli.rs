use std::path::PathBuf;

use clap::{Parser, Subcommand};
use jiff::civil::Date;

#[derive(clap::ValueEnum, Clone)]
pub enum GroupBy {
    Day,
    Month,
    Year,
}

#[derive(Parser)]
#[command(name = "blok", version)]
pub struct Cli {
    #[arg(short, long, default_value = "blok.log", env = "BLOK_FILE")]
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new entry
    Add {
        /// Date in YYYY-MM-DD format, defaults to today
        #[arg(long, value_parser = parse_date)]
        date: Option<Date>,
        duration: String,
        tag: String,
        note: Option<String>,
    },
    /// Print entries
    Print {
        #[arg(long)]
        tag: Option<String>,
        #[arg(long, value_parser = parse_date)]
        from: Option<Date>,
        #[arg(long, value_parser = parse_date)]
        to: Option<Date>,
    },
    /// Create a table report
    Report {
        #[arg(long, default_value = "day", value_enum)]
        group_by: GroupBy,
        #[arg(long, value_parser = parse_date)]
        from: Option<Date>,
        #[arg(long, value_parser = parse_date)]
        to: Option<Date>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        include_empty: bool,
    },
}

fn parse_date(arg: &str) -> Result<Date, String> {
    arg.parse()
        .map_err(|_| "invalid date, expected YYYY-MM-DD".to_string())
}
