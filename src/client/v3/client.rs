use std::future::Future;
use truelayer_rust::{TrueLayerClient, apis::auth::Credentials};
use truelayer_rust::apis::auth::Token;
use truelayer_rust::client::Environment;
use crate::client::v3::payment::authorizationflow::provider_selection::submit_provider_selection;
use crate::client::v3::payment::authorizationflow::start::start_authorization_flow;
use crate::client::v3::payment::create::create_payment;
use crate::client::v3::payment::mockprovider::execute_payment;

fn new_truelayer_client(client_id: String, client_secret: Token, private_key: Vec<u8>) -> truelayer_rust::TrueLayerClient {
    TrueLayerClient::builder(Credentials::ClientCredentials {
        client_id,
        client_secret,
        scope: "payments".into(),
    })
        .with_signing_key("my-kid", private_key)
        .with_environment(Environment::Sandbox)
        .build()
}

pub fn new(client_id: String, client_secret: String, private_key: String) -> impl Client {
    ClientImpl{
        truelayer_client: new_truelayer_client(
            client_id,
            client_secret.into(),
            private_key.into()
        )
    }
}

trait Client {
    fn create_payment(&self) -> Box<dyn Future<Output = anyhow::Result<String>>>;
    fn start_authorization(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>>;
    fn submit_gb_provider(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>>;
    fn execute_payment(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>>;
}

struct ClientImpl {
    truelayer_client: TrueLayerClient,
}

impl Client for ClientImpl {
    fn create_payment(&self) -> Box<dyn Future<Output = anyhow::Result<String>>> {
         Box::new(create_payment(&self.truelayer_client) )
    }

    fn start_authorization(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(start_authorization_flow(payment_id, &self.truelayer_client))
    }

    fn submit_gb_provider(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(submit_provider_selection(payment_id, &self.truelayer_client))
    }

    fn execute_payment(&self, payment_id: &String) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(execute_payment(payment_id))
    }
}