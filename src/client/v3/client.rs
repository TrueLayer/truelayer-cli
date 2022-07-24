use crate::client::v3::payment::authorizationflow::start::start_authorization_flow;
use crate::client::v3::payment::create::create_merchant_account_payment;
use crate::client::v3::payment::mockprovider::execute_payment;
use truelayer_rust::apis::auth::Token;
use truelayer_rust::client::Environment;
use truelayer_rust::{apis::auth::Credentials, TrueLayerClient};

fn new_truelayer_client(
    client_id: String,
    client_secret: Token,
    kid: String,
    private_key: Vec<u8>,
) -> truelayer_rust::TrueLayerClient {
    TrueLayerClient::builder(Credentials::ClientCredentials {
        client_id,
        client_secret,
        scope: "payments".into(),
    })
    .with_signing_key(kid.as_ref(), private_key)
    .with_environment(Environment::Sandbox)
    .build()
}

pub fn new(client_id: String, client_secret: String, kid: String, private_key: String) -> Client {
    Client {
        truelayer_client: new_truelayer_client(
            client_id,
            client_secret.into(),
            kid,
            private_key.into(),
        ),
    }
}

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
}
