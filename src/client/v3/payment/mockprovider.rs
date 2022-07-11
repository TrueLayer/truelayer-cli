use reqwest::{Error, Response};
use serde_json::json;

// const PROVIDER_ACTION: &str = "https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action";

pub async fn execute_payment(payment_id: &str) -> anyhow::Result<()> {
    let mock_provider_endpoint = format!("https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action", payment_id);
    match reqwest::Client::new()
        .post(mock_provider_endpoint)
        .json(&json!({
            "action": "Execute",
            "redirect": false
        }))
        .send()
        .await {
        Ok(response) => Ok(()),
        Err(e) => anyhow::bail!(e)
    }
}