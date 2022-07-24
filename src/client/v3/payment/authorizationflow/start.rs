use anyhow::Error;
use truelayer_rust::apis::payments::{
    AuthorizationFlowNextAction, AuthorizationFlowResponseStatus, ProviderSelectionSupported,
    RedirectSupported, StartAuthorizationFlowRequest,
};

// Returns uri
pub async fn start_authorization_flow(
    payment_id: &str,
    client: &truelayer_rust::TrueLayerClient,
) -> anyhow::Result<String> {
    let response = client
        .payments
        .start_authorization_flow(
            payment_id,
            &StartAuthorizationFlowRequest {
                provider_selection: Some(ProviderSelectionSupported {}),
                redirect: Some(RedirectSupported {
                    return_uri: "http://localhost:3000/callback".to_string(),
                    direct_return_uri: None,
                }),
                form: None,
            },
        )
        .await?;
    match response.status {
        AuthorizationFlowResponseStatus::Authorizing => {
            match response
                .authorization_flow
                .ok_or_else(|| Error::msg("Authorization flow object not found"))?
                .actions
                .ok_or_else(|| Error::msg("Actions in authorization flow not found"))?
                .next
            {
                AuthorizationFlowNextAction::Redirect { uri, metadata: _ } => Ok(uri),
                _ => Err(Error::msg(
                    "Next action is not redirect, there is a problem with the flow",
                )),
            }
        }
        AuthorizationFlowResponseStatus::Failed {
            failure_stage: _,
            failure_reason,
        } => Err(Error::msg(format!(
            "Authorization flow was not started successfully. Reason: {}",
            failure_reason
        ))),
    }
}
