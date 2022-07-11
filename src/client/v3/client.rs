use truelayer_rust::{TrueLayerClient, apis::auth::Credentials};
use truelayer_rust::apis::auth::Token;
use truelayer_rust::client::Environment;

pub fn new_client(client_id: String, client_secret: Token, private_key: Vec<u8>) -> truelayer_rust::TrueLayerClient {
    TrueLayerClient::builder(Credentials::ClientCredentials {
        client_id,
        client_secret,
        scope: "payments".into(),
    })
        .with_signing_key("my-kid", private_key)
        .with_environment(Environment::Sandbox)
        .build()
}