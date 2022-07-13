use crate::client::v3::client::Client;
use crate::client::v3::client::new as new_client;

pub struct Commander {
    client: Client,
}

pub fn new(client_id: String, client_secret: String, kid: String, private_key: String) -> Commander {
    return Commander {
        client: new_client(client_id, client_secret, kid, private_key)
    };
}

impl Commander {
    pub async fn generate_settled_event(&self) {
        println!("Creating payment");
        let payment_id = self.client.create_payment().await.expect("Error while creating payment");
        println!("Created payment with id {}", payment_id);
        println!("Starting auth");
        self.client.start_authorization(&payment_id).await.expect("Error while starting authorization flow");
        println!("Executing payment");
        self.client.execute_payment(&payment_id).await.expect("Error while executing the payment");
        ()
    }

    fn generate_failed_event() -> anyhow::Result<()> {
        todo!()
    }
}