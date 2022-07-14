use anyhow::Error;
use truelayer_rust::apis::payments::{
    AuthorizationFlowResponseStatus, SubmitProviderSelectionActionRequest,
    SubmitProviderSelectionActionResponse,
};
use truelayer_rust::TrueLayerClient;

const GB_MOCK_PROVIDER_ID: &str = "mock-payments-gb-redirect";

pub async fn submit_provider_selection(
    payment_id: &str,
    client: &TrueLayerClient,
) -> anyhow::Result<()> {
    let resp = client
        .payments
        .submit_provider_selection(
            payment_id,
            &SubmitProviderSelectionActionRequest {
                provider_id: GB_MOCK_PROVIDER_ID.to_string(),
            },
        )
        .await?;
    match resp.status {
        AuthorizationFlowResponseStatus::Authorizing => Ok(()),
        AuthorizationFlowResponseStatus::Failed {
            failure_stage,
            failure_reason,
        } => Err(Error::msg(format!(
            "Authorization flow was not started successfully. Reason: {}",
            failure_reason
        ))),
    }
}
