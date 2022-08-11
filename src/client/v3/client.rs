use std::str::FromStr;

use anyhow::Error;
use truelayer_rust::apis::auth::Token;
use truelayer_rust::client::Environment;
use truelayer_rust::{apis::auth::Credentials, TrueLayerClient};

use url::Url;

use crate::client::v3::payment::authorizationflow::start::start_authorization_flow;
use crate::client::v3::payment::create::create_merchant_account_payment;
use crate::client::v3::payment::mockprovider::{execute_payment, fail_authorization};

pub fn new(client_id: String, client_secret: Token, kid: String, private_key: Vec<u8>) -> Client {
    let tl_client = TrueLayerClient::builder(Credentials::ClientCredentials {
        client_id,
        client_secret,
        scope: "payments".into(),
    })
    .with_signing_key(kid.as_ref(), private_key)
    .with_environment(Environment::Sandbox)
    .build();
    Client {
        truelayer_client: tl_client,
    }
}

pub fn new_auth_client(client_id: String, client_secret: Token) -> Client {
    let tl_client = TrueLayerClient::builder(Credentials::ClientCredentials {
        client_id,
        client_secret,
        scope: "payments".into(),
    })
    .with_environment(Environment::Sandbox)
    .build();
    Client {
        truelayer_client: tl_client,
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    truelayer_client: TrueLayerClient,
}

impl Client {
    pub async fn create_merchant_account_payment(&self) -> anyhow::Result<String> {
        create_merchant_account_payment(&self.truelayer_client).await
    }

    pub async fn start_authorization(&self, payment_id: &str) -> anyhow::Result<String> {
        start_authorization_flow(payment_id, &self.truelayer_client).await
    }

    pub async fn execute_payment(&self, payment_id: &str, token: &str) -> anyhow::Result<()> {
        execute_payment(payment_id, token).await
    }

    pub async fn fail_payment(&self, payment_id: &str, token: &str) -> anyhow::Result<()> {
        fail_authorization(payment_id, token).await
    }

    pub async fn get_token(&self) -> anyhow::Result<String> {
        self.truelayer_client
            .auth
            .get_access_token()
            .await
            .map(|r| r.access_token().token().expose_secret().to_string())
            .map_err(|e| Error::new(e))
    }
}
