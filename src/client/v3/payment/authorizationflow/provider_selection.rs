use truelayer_rust::apis::payments::{AuthorizationFlowResponseStatus, SubmitProviderSelectionActionRequest, SubmitProviderSelectionActionResponse};
use truelayer_rust::{Error, TrueLayerClient};

const GB_MOCK_PROVIDER_ID: &str = "mock-payments-gb-redirect";

pub async fn submit_provider_selection(payment_id: &str, client: &TrueLayerClient) -> anyhow::Result<()> {
    match client.payments.submit_provider_selection(payment_id, &SubmitProviderSelectionActionRequest{
       provider_id: GB_MOCK_PROVIDER_ID.to_string()
    }).await {
        Ok(resp) => {
            match resp.status {
                AuthorizationFlowResponseStatus::Authorizing => Ok(()),
                AuthorizationFlowResponseStatus::Failed { failure_stage, failure_reason } => anyhow::bail!("Errored during start authorization flow with reason: {}", failure_reason)
            }
        }
        Err(e) => anyhow::bail!(e)
    }
}