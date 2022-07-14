use anyhow::Error;
use regex::Regex;
use reqwest::Url;
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

fn extract_token_from_uri(uri:&String) -> anyhow::Result<String> {
    let parsed_uri = Url::parse(uri.as_ref())?;
    let re = Regex::new(r"token=(.+)")?;
    re.captures(parsed_uri.fragment().ok_or(Error::msg("No fragment present in the redirect uri"))?)
        .ok_or(Error::msg("Could not capture token in the fragment"))?
        .get(1)
        .map_or(Err(Error::msg("No token found in the uri")), |m| Ok(m.as_str().into()))
}

fn extract_mock_payment_id(uri:&String) -> anyhow::Result<String> {
    let parsed_uri = Url::parse(uri.as_ref())?;
    let path = String::from(parsed_uri.path());
    let mock_id = path.split("/").last().ok_or(Error::msg("Could not get payment id from uri"))?;
    Ok(String::from(mock_id))
}

impl Commander {
    pub async fn generate_settled_event(&self) {
        println!("Creating payment");
        let payment_id = self.client.create_merchant_account_payment().await.expect("Error while creating payment");
        println!("Created payment with id {}", payment_id);
        println!("Starting auth");
        let uri = self.client.start_authorization(&payment_id).await.expect("Error while starting authorization flow");
        let mock_payment_id = extract_mock_payment_id(&uri).expect("Could not extract mock payment id");
        let token = extract_token_from_uri(&uri);
        println!("Mock payment_id: {}", mock_payment_id);
        println!("Executing payment");
        self.client.execute_payment(&mock_payment_id, &token.unwrap()).await.expect("Error while executing the payment");
        ()
    }

    fn generate_failed_event() -> anyhow::Result<()> {
        todo!()
    }
}