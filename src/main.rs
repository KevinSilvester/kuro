use clap::Parser;

mod cli;
mod config;

fn main() {
    let value =  cli::Cli::parse();
    dbg!(value);
}
