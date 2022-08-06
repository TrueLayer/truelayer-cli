use std::sync::Arc;
use std::time::SystemTime;

use anyhow::Error;
use chrono::Duration;
use colored::Colorize;
use futures_util::lock::Mutex;
use json::JsonValue;
use reqwest::Response;
use serde::Deserialize;
use timer;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::client::v3::client::Client as v3_client;
use crate::platform::model::{PullResponse, Webhook};
use crate::platform::{client, model};

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub const WEBHOOK_ROUTER_URI: &str = "webhooks";

pub struct Client {
    fetcher: Fetcher,
}

impl Client {
    pub async fn init(client: v3_client, route_to: String) -> anyhow::Result<Self> {
        let token = client.get_token().await?;
        let fetcher = Fetcher { token, route_to };

        Ok(Client { fetcher })
    }

    pub async fn start(&mut self) -> JoinHandle<()> {
        tokio::spawn(async {})
    }
}

struct Fetcher {
    token: String,
    route_to: String,
}

impl Fetcher {
    fn send_webhooks(&self, webhooks: Vec<Webhook>) -> anyhow::Result<()> {
        for wh in webhooks.iter() {
            let builder = reqwest::Client::new()
                .post(&self.route_to)
                .header("authorization", &self.token);

            for (k, v) in wh.headers.iter() {
                builder.header(k, v);
            }

            match builder.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!(
                            "{} {}",
                            "A webhook was successfully routed to address".green(),
                            self.route_to.cyan()
                        );
                    } else {
                        println!(
                            "{} {} {} {}",
                            "A webhook has failed to be routed to address ".yellow(),
                            self.route_to.cyan(),
                            " with status code: ",
                            resp.status()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{} {} {} {}",
                        "HTTP request to the route address ".red(),
                        self.route_to.cyan(),
                        " has failed, with error: ",
                        e
                    );
                }
            }
        }

        Ok(())
    }

    fn fetch(&self) -> anyhow::Result<()> {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        loop {
            match client
                .get(WEBHOOK_ROUTER_URI)
                .header("authorization", &self.token)
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let json: model::PullResponse = resp.json().await?;
                        self.send_webhooks(json.webhooks)
                    }
                    break;
                }
                Err(e) => {
                    println!(
                        "{} {} {}",
                        "HTTP request to the server ".bright_red(),
                        self.route_to.cyan(),
                        " has failed. If it continues, restart the CLI program"
                    );
                }
            };

            tokio::time::sleep(Duration::from_millis(10000)).await;
        }

        Err(Error::msg("Exited the loop prematurely"))
    }
}
