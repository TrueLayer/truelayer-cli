use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullResponse {
    pub webhooks: Vec<Webhook>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookMessage {
    pub event_id: String,
    pub payment_id: String,
    #[serde(rename = "type")]
    pub typ: String,
}
