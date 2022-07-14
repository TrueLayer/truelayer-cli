use anyhow::Error;
use regex::Regex;
use reqwest::Url;
use truelayer_rust::apis::payments::{
    AuthorizationFlowNextAction, AuthorizationFlowResponseStatus, ProviderSelectionSupported,
    RedirectSupported, StartAuthorizationFlowRequest, StartAuthorizationFlowResponse,
};

// Returns uri
pub async fn start_authorization_flow(
    payment_id: &String,
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
                .ok_or(Error::msg("Authorization flow object not found"))?
                .actions
                .ok_or(Error::msg("Actions in authorization flow not found"))?
                .next
            {
                AuthorizationFlowNextAction::Redirect { uri, metadata } => Ok(uri),
                _ => Err(Error::msg(
                    "Next action is not redirect, there is a problem with the flow",
                )),
            }
        }
        AuthorizationFlowResponseStatus::Failed {
            failure_stage,
            failure_reason,
        } => Err(Error::msg(format!(
            "Authorization flow was not started successfully. Reason: {}",
            failure_reason
        ))),
    }
}
