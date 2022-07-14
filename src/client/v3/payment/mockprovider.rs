use reqwest::{Response};
use anyhow::Error;
use serde_json::json;

// const PROVIDER_ACTION: &str = "https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action";

pub async fn execute_payment(payment_id: &str, token: &str) -> anyhow::Result<()> {
    let mock_provider_endpoint = format!("https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action", payment_id);
    match reqwest::Client::new()
        .post(mock_provider_endpoint)
        .json(&json!({
            "action": "Execute",
            "redirect": false
        }))
        .header("authority", "pay-mock-connect.truelayer-sandbox.com")
        .header("scheme", "https")
        .bearer_auth(token)
        .send()
        .await {
        Ok(response) => {
            if !response.status().is_success() {
               return Err(Error::msg(format!("Execute payment: status was {}", response.status().as_str())))
            }
            Ok(())
        },
        Err(e) => anyhow::bail!(e)
    }
}