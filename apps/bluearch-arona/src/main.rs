mod banner;
mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::About) => commands::about::run(),
        Some(Commands::Version) => commands::version::run(),
        None => banner::show(),
    }
}
