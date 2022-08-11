use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::Error;
use colored::Colorize;
use futures_util::lock::Mutex;
use json::JsonValue;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::Deserialize;
use timer;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::client::v3;
use crate::platform::model::{PullResponse, Webhook};
use crate::platform::{client, model};

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub const WEBHOOK_ROUTER_URI: &str = "https://webhook-router.truelayer-sandbox.com/pull";

pub struct WebhookRouterClient {
    token: String,
    route_to: String,
}

impl WebhookRouterClient {
    pub async fn init(client: v3::client::Client, route_to: String) -> anyhow::Result<Self> {
        let token = client.get_token().await?;

        Ok(WebhookRouterClient { token, route_to })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let runner = Runner {
            token: self.token.clone(),
            route_to: self.route_to.clone(),
        };

        match runner.fetch().await {
            Ok(_) => Err(Error::msg("Fetching Prematurely exited")),
            Err(e) => Err(e),
        }
    }
}

struct Runner {
    token: String,
    route_to: String,
}

impl Runner {
    async fn send_webhooks(&self, webhooks: Vec<Webhook>) -> anyhow::Result<()> {
        let webhooks_c = webhooks.clone();
        for wh in webhooks_c.iter() {
            println!("webhook body: {}", wh.body);
            let mut builder = reqwest::Client::new()
                .post(&self.route_to)
                .bearer_auth(&self.token);

            for (k, v) in wh.headers.iter() {
                builder = builder.header(k.clone(), v.clone());
            }

            match builder
                // match builder
                .send()
                .await
            {
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

    pub async fn fetch(&self) -> anyhow::Result<()> {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        println!("{}", "Pulling webhooks...".yellow());
        loop {
            match client
                .get(WEBHOOK_ROUTER_URI)
                .bearer_auth(&self.token)
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let json: model::PullResponse = resp.json().await?;
                        self.send_webhooks(json.webhooks).await;
                    } else {
                        println!(
                            "{}{}",
                            "Unexpected status code: ".red(),
                            resp.status().to_string().cyan()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{} {} {} {}",
                        "HTTP request to the server ".bright_red(),
                        self.route_to.cyan(),
                        " has failed. If it continues, restart the CLI program, error: ",
                        e
                    );
                }
            };

            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        Err(Error::msg("Exited the loop prematurely"))
    }
}
