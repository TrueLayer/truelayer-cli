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
    CreateTunnel {
        #[clap(long, value_parser)]
        route_to: String,
    },

    TlCreateTunnel {
        #[clap(long, value_parser)]
        route_to: String,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateWehookMode {
    ExecutedSettled {},
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
                GenerateWehookMode::ExecutedSettled {} => {
                    match commander.generate_settled_event().await {
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
        Commands::CreateTunnel { route_to } => {
            match commands::commander::new().create_tunnel(route_to).await {
                Ok(_) => println!("Was okay"),
                Err(e) => panic!("{}", e),
            }
        }
        Commands::TlCreateTunnel { route_to } => {
            match commands::commander::new().create_tunnel(route_to).await {
                Ok(_) => println!("Was okay"),
                Err(e) => panic!("{}", e),
            }
        }
    }
}
