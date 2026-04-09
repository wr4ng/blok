use blok::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    blok::run(cli);
}
