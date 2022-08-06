use crate::client::v3::client;
use crate::client::v3::client::new as new_client;
use crate::client::v3::client::Client;
use crate::platform::client::Client as AsyncClient;
use anyhow::{Context, Error};
use colored::Colorize;
use regex::Regex;
use reqwest::Url;
use std::process::Stdio;
use std::str;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use truelayer_rust::{apis::auth::Credentials, TrueLayerClient};

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
        client: Some(new_client(client_id, client_secret, kid, private_key)),
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

    pub async fn create_tunnel(&self, route_to: String) -> anyhow::Result<()> {
        let mut child = Command::new("cloudflared")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("tunnel")
            .arg("--url")
            .arg(route_to.as_str())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stderr.take().context("No stderr found")?;
        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            println!("{}", "Creating a tunnel".yellow());
            let status = child
                .wait()
                .await
                .expect("Child process encountered an error");

            println!("child status was: {}", status);
        });

        let mut tunnel_created = false;
        let mut line_counter = 0;
        while let Some(line) = reader.next_line().await.unwrap() {
            line_counter += 1;
            // It has been 20 lines and we still have not found the URL, we should bail
            if line_counter == 20 && !tunnel_created {
                return Err(Error::msg("Could not create a public tunnel url"));
            }
            if !tunnel_created {
                match extract_url(&line) {
                    Ok(url) => {
                        println!("{} {}", "Created tunnel with url :".green(), url.cyan());
                        tunnel_created = true;
                    }
                    Err(_e) => {}
                };
            } else {
                println!("{}", line.yellow())
            }
        }

        Ok(())
    }

    pub async fn create_tl_tunnel(&self, route_to: String) -> anyhow::Result<()> {
        AsyncClient::init("localhost:7000".into(), self.client.as_ref().unwrap()).await?
    }
}
