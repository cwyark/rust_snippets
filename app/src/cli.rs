use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "hello!")]
    message: String,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    println!("{}", message.bright_yellow().underline().bold());
}
