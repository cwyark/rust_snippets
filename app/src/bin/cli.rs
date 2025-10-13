use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
struct Options {
    message: String,

    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add files to this app
    Add { name: Option<String> },
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    println!("{}", message.bright_yellow().underline().bold());
}
