mod client;
mod commands;
mod platform;
use clap::{Parser, Subcommand};
use colored::Colorize;
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
    RouteWebhooks {
        #[clap(long, value_parser)]
        to_addr: String,
        #[clap(long, value_parser)]
        client_secret: String,
        #[clap(long, value_parser)]
        client_id: String,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateWehookMode {
    Executed {},
    Failed {},
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
            let commander = commands::commander::new_with_client(
                client_id,
                client_secret,
                kid,
                contents.expect("Error while reading key content"),
            );

            match mode {
                GenerateWehookMode::Executed {} => {
                    match commander.generate_executed_event().await {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e.to_string().as_str().red()),
                    }
                }
                GenerateWehookMode::Failed {} => match commander.generate_failed_event().await {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e.to_string().as_str().red()),
                },
            }
        }
        Commands::RouteWebhooks {
            to_addr,
            client_id,
            client_secret,
        } => {
            let commander = commands::commander::new_with_auth_client(client_id, client_secret);
            match commander.create_tunnel(to_addr).await {
                Ok(_) => println!("Was okay"),
                Err(e) => panic!("{}", e),
            }
        }
    }
}
