use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookRequest {
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub typ: MessageType,
    pub payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    WebhookRequest,
    Ping,
}
