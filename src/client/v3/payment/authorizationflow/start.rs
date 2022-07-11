use truelayer_rust::apis::payments::{AuthorizationFlowResponseStatus, ProviderSelectionSupported, RedirectSupported, StartAuthorizationFlowRequest, StartAuthorizationFlowResponse};
use anyhow::Error;

pub async fn start_authorization_flow(payment_id: &String, client: &truelayer_rust::TrueLayerClient) -> anyhow::Result<()> {
   match client.payments.start_authorization_flow(payment_id, &StartAuthorizationFlowRequest{
       provider_selection: Some(ProviderSelectionSupported{}),
       redirect: Some(RedirectSupported{
           return_uri: "localhost:3000".to_string(),
           direct_return_uri: None
       }),
       form: None
   }).await {
       Ok(response) => {
           match response.status {
               AuthorizationFlowResponseStatus::Authorizing => Ok(()),
               AuthorizationFlowResponseStatus::Failed { .. } => panic!("Errored during start authorization flow")
           }
       },
       Err(e) => panic!("{:?}", e)
   }
}