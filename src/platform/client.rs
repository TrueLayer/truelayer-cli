use std::time::Duration;

use anyhow::Error;
use colored::Colorize;

use crate::client::v3;
use crate::platform::model;
use crate::platform::model::{Webhook, WebhookMessage};

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub const WEBHOOK_ROUTER_URI: &str = "https://webhook-router.truelayer-sandbox.com/pull";

pub struct WebhookRouterClient {
    token: String,
    addr: String,
}

impl WebhookRouterClient {
    pub async fn init(client: v3::client::Client, addr: String) -> anyhow::Result<Self> {
        let token = client.get_token().await?;

        Ok(WebhookRouterClient { token, addr: addr })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let runner = Runner {
            token: self.token.clone(),
            addr: self.addr.clone(),
        };

        match runner.fetch().await {
            Ok(_) => Err(Error::msg("Fetching Prematurely exited")),
            Err(e) => Err(e),
        }
    }
}

struct Runner {
    token: String,
    addr: String,
}

impl Runner {
    async fn send_webhooks(&self, webhooks: Vec<Webhook>) {
        let webhooks_c = webhooks.clone();
        for wh in webhooks_c.iter() {
            let webhook_message: WebhookMessage = match serde_json::from_str::<WebhookMessage>(wh.body.clone().as_str()) {
                Ok(wm) => wm,
                Err(e) => {
                    println!("Error while deserialising webhook message {}", e);
                    continue;
                }
            };

            let mut builder = reqwest::Client::new()
                .post(&self.addr)
                .body(wh.body.clone())
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
                            "{} {}, {} {}, {} {}, => {}, {}",
                            "Type: ",
                            webhook_message.typ.cyan(),
                            "Event id:",
                            webhook_message.event_id.cyan(),
                            "Payment id:",
                            webhook_message.payment_id.cyan(),
                            self.addr.cyan(),
                            "SUCCESS".green()
                        );
                    } else {
                        println!(
                            "{} {}, {} {}, {} {}, => {}, {}, {} {}",
                            "Type: ",
                            webhook_message.typ.cyan(),
                            "Event id:",
                            webhook_message.event_id.cyan(),
                            "Payment id:",
                            webhook_message.payment_id.cyan(),
                            self.addr.cyan(),
                            "FAILURE".red(),
                            "status:",
                            resp.status()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{} {}, {} {}, {} {}, => {}, {}, {} {}",
                        "Type: ",
                        webhook_message.typ.cyan(),
                        "Event id:",
                        webhook_message.event_id.cyan(),
                        "Payment id:",
                        webhook_message.payment_id.cyan(),
                        self.addr.cyan(),
                        "FAILURE".red(),
                        "error:",
                        e
                    );
                }
            }
        }
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
                        "{} {}  has failed. If it continues, restart the CLI program, error:  {}",
                        "HTTP request to the server ".bright_red(),
                        self.addr.cyan(),
                        e
                    );
                }
            };

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
