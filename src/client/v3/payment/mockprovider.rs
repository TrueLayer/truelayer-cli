use anyhow::Error;
use serde_json::json;

// const PROVIDER_ACTION: &str = "https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action";

async fn submit_action(payment_id: &str, token: &str, action: &str) -> anyhow::Result<()> {
    let mock_provider_endpoint = format!(
        "https://pay-mock-connect.truelayer-sandbox.com/api/single-immediate-payments/{}/action",
        payment_id
    );
    match reqwest::Client::new()
        .post(mock_provider_endpoint)
        .json(&json!({
            "action": action,
            "redirect": false
        }))
        .header("authority", "pay-mock-connect.truelayer-sandbox.com")
        .header("scheme", "https")
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(Error::msg(format!(
                    "Execute payment: status was {}",
                    response.status().as_str()
                )));
            }
            Ok(())
        }
        Err(e) => anyhow::bail!(e),
    }
}

pub async fn execute_payment(payment_id: &str, token: &str) -> anyhow::Result<()> {
    submit_action(payment_id, token, "Execute").await
}

pub async fn fail_authorization(payment_id: &str, token: &str) -> anyhow::Result<()> {
    submit_action(payment_id, token, "RejectAuthorisation").await
}
