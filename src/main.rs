mod client;
pub mod commands;

use std::fs;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, value_parser)]
    pub private_key: String,
    #[clap(long, value_parser)]
    pub client_secret: String,
    #[clap(long, value_parser)]
    pub client_id: String,
    #[clap(long, value_parser)]
    pub kid: String,
    #[clap(long, value_parser)]
    pub mode: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.private_key);
    // println!("{}", contents.expect("no"));
    let commander = commands::commander::new(args.client_id, args.client_secret, args.kid, contents.expect("Error while reading key content"));

    match args.mode.as_ref() {
        "a" => commander.generate_settled_event().await,
        &_ => panic!("Some error")
    }
}
