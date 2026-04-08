use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(clap::ValueEnum, Clone)]
pub enum GroupBy {
    Day,
    Month,
    Year,
}

#[derive(Parser)]
#[command(name = "blok", version)]
pub struct Cli {
    #[arg(short, long, default_value = "blok.log")]
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new entry
    Add {
        /// Date in YYYY-MM-DD format, defaults to today
        #[arg(long)]
        date: Option<String>,
        duration: String,
        tag: String,
        note: Option<String>,
    },
    /// Print entries
    Print {
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    /// Create a table report
    Report {
        #[arg(long, default_value = "day", value_enum)]
        group_by: GroupBy,
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        include_empty: bool, //TODO: include_between instead?
    },
}
