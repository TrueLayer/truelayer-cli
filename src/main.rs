mod client;
mod commands;

use std::fs;
use clap::{Parser, Subcommand};
use reqwest::Url;
use regex::Regex;

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateWebhook {
        #[clap(long, value_parser)]
        private_key: String,
        #[clap(long, value_parser)]
        client_secret: String,
        #[clap(long, value_parser)]
        client_id: String,
        #[clap(long, value_parser)]
        kid: String,
        #[clap(long, value_parser)]
        mode: String,    
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Sub::GenerateWebhook {
            private_key,
            client_secret,
            client_id,
            kid,
            mode
        } => {
            let contents = fs::read_to_string(&private_key);
            let commander = commands::commander::new(client_id, client_secret, kid, contents.expect("Error while reading key content"));

            let mode = mode.as_str();
            match mode {
                "executed-settled" => commander.generate_settled_event().await,
                &_ => panic!("incorrect mode passed {}", mode)
            }
        }
    }

}
