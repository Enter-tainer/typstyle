use clap::Parser;

use crate::cli::CliArguments;

mod cli;

fn main() {
    let CliArguments { input } = CliArguments::parse();
    println!("Hello, world!");
}
