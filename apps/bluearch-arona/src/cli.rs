use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "arona",
    version,
    about = "BlueArch system assistant",
    long_about = "Arona is the command-line system assistant for BlueArch Linux."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display information about Arona and BlueArch
    About,

    /// Display detailed version information
    Version,
}
