mod client;
mod commands;
use clap::{Parser, Subcommand};
use colored::Colorize;
use regex::Regex;
use reqwest::Url;
use std::fs;

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateWebhook {
        #[clap(subcommand)]
        mode: GenerateWehookMode,
        #[clap(long, value_parser)]
        private_key: String,
        #[clap(long, value_parser)]
        client_secret: String,
        #[clap(long, value_parser)]
        client_id: String,
        #[clap(long, value_parser)]
        kid: String,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateWehookMode {
    ExecutedSettled {},
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
        Commands::GenerateWebhook {
            private_key,
            client_secret,
            client_id,
            kid,
            mode,
        } => {
            let contents = fs::read_to_string(&private_key);
            let commander = commands::commander::new(
                client_id,
                client_secret,
                kid,
                contents.expect("Error while reading key content"),
            );

            match mode {
                GenerateWehookMode::ExecutedSettled {} => {
                    match commander.generate_settled_event().await {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e.to_string().as_str().red()),
                    }
                }
            }
        }
    }
}
