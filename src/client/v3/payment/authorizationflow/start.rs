use anyhow::Error;
use regex::Regex;
use reqwest::Url;
use truelayer_rust::apis::payments::{AuthorizationFlowNextAction, AuthorizationFlowResponseStatus, ProviderSelectionSupported, RedirectSupported, StartAuthorizationFlowRequest, StartAuthorizationFlowResponse};

// Returns uri
pub async fn start_authorization_flow(payment_id: &String, client: &truelayer_rust::TrueLayerClient) -> anyhow::Result<String> {
    match client.payments.start_authorization_flow(payment_id, &StartAuthorizationFlowRequest {
        provider_selection: Some(ProviderSelectionSupported {}),
        redirect: Some(RedirectSupported {
            return_uri: "http://localhost:3000/callback".to_string(),
            direct_return_uri: None,
        }),
        form: None,
    }).await {
        Ok(response) => {
            match response.status {
                AuthorizationFlowResponseStatus::Authorizing => {
                    match response.authorization_flow
                        .ok_or(Error::msg("Authorization flow object not found"))?
                        .actions
                        .ok_or(Error::msg("Actions in authorization flow not found"))?
                        .next {
                        AuthorizationFlowNextAction::Redirect { uri, metadata } => {
                            Ok(uri)
                        }
                        _ => Err(Error::msg("Next action is not redirect, there is a problem with the flow"))
                    }
                }
                AuthorizationFlowResponseStatus::Failed { .. } => panic!("Errored during start authorization flow")
            }
        }
        Err(e) => panic!("{:?}", e)
    }
}