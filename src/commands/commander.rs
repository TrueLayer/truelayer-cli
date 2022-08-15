use crate::client::v3::client::Client;
use crate::client::v3::client::{new as new_client, new_auth_client};
use crate::platform::client::WebhookRouterClient;
use anyhow::{Context, Error};
use colored::Colorize;
use regex::Regex;
use reqwest::Url;
use std::process::Stdio;
use std::str;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub struct Commander {
    client: Option<Client>,
}

pub fn new_with_client(
    client_id: String,
    client_secret: String,
    kid: String,
    private_key: String,
) -> Commander {
    Commander {
        client: Some(new_client(
            client_id,
            client_secret.into(),
            kid,
            private_key.into(),
        )),
    }
}

pub fn new_with_auth_client(client_id: String, client_secret: String) -> Commander {
    Commander {
        client: Some(new_auth_client(client_id, client_secret.into())),
    }
}

pub fn new() -> Commander {
    Commander { client: None }
}

fn extract_token_from_uri(uri: &str) -> anyhow::Result<String> {
    let parsed_uri = Url::parse(uri)?;
    let re = Regex::new(r"token=(.+)")?;
    re.captures(
        parsed_uri
            .fragment()
            .ok_or_else(|| Error::msg("No fragment present in the redirect uri"))?,
    )
    .ok_or_else(|| Error::msg("Could not capture token in the fragment"))?
    .get(1)
    .map_or(Err(Error::msg("No token found in the uri")), |m| {
        Ok(m.as_str().into())
    })
}

fn extract_mock_payment_id(uri: &str) -> anyhow::Result<String> {
    let parsed_uri = Url::parse(uri)?;
    let path = String::from(parsed_uri.path());
    let mock_id = path
        .split('/')
        .last()
        .ok_or_else(|| Error::msg("Could not get payment id from uri"))?;
    Ok(String::from(mock_id))
}

fn extract_url(line: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"\|  (https.+)\s")?;
    re.captures(line)
        .ok_or_else(|| Error::msg("Could not capture https pattern"))?
        .get(1)
        .map_or(Err(Error::msg("No urls found")), |m| Ok(m.as_str().into()))
}

impl Commander {
    async fn create_auth_uri(&self) -> anyhow::Result<String> {
        println!("{}", "Creating merchant account payment".yellow());
        let payment_id = self
            .client
            .as_ref()
            .unwrap()
            .create_merchant_account_payment()
            .await
            .context("Error while creating merchant account payment")?;
        println!(
            "{} {}",
            "Created payment with id".green(),
            payment_id.as_str().cyan()
        );

        println!("{}", "Starting auth flow".yellow());
        self.client
            .as_ref()
            .unwrap()
            .start_authorization(&payment_id)
            .await
            .context("Error while starting authorization flow")
    }

    pub async fn generate_settled_event(&self) -> anyhow::Result<()> {
        let uri = self.create_auth_uri().await?;
        println!("{}", "Authflow successfully started".green());

        let mock_payment_id =
            extract_mock_payment_id(&uri).context("Could not extract mock payment id")?;
        let token = extract_token_from_uri(&uri);
        println!(
            "{} {}",
            "Mock payment_id: ".green(),
            mock_payment_id.as_str().cyan()
        );

        println!("{}", "Executing payment".yellow());
        self.client
            .as_ref()
            .unwrap()
            .execute_payment(&mock_payment_id, &token.unwrap())
            .await
            .context("Error while executing the payment")?;
        println!("{}", "Payment executed".green());
        println!("{}", "Completed".green());
        Ok(())
    }

    pub async fn generate_failed_event(&self) -> anyhow::Result<()> {
        let uri = self.create_auth_uri().await?;
        println!("{}", "Authflow successfully started".green());

        let mock_payment_id =
            extract_mock_payment_id(&uri).context("Could not extract mock payment id")?;
        let token = extract_token_from_uri(&uri);
        println!(
            "{} {}",
            "Mock payment_id: ".green(),
            mock_payment_id.as_str().cyan()
        );

        println!("{}", "Failing payment".yellow());
        self.client
            .as_ref()
            .unwrap()
            .fail_payment(&mock_payment_id, &token.unwrap())
            .await
            .context("Error while failing the payment")?;
        println!("{}", "Payment failure executed".green());
        println!("{}", "Completed".green());
        Ok(())
    }

    pub async fn create_tunnel(&self, addr: String) -> anyhow::Result<()> {
        let mut webhook_router_client =
            WebhookRouterClient::init(self.client.as_ref().unwrap().clone(), addr).await?;
        webhook_router_client.start().await
    }
}
