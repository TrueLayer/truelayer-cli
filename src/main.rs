mod client;
pub mod commands;

use std::fs;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    private_key: String,
    #[clap(short, long, value_parser)]
    mode: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.private_key)
        .expect("Could not read the private key file");
    println!("{}", args.private_key);
    println!("{}", contents);
}
