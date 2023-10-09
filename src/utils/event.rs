use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    #[serde(rename = "chat")]
    Chat(String),
    #[serde(rename = "speech")]
    Speech(String),
    #[serde(rename = "loading")]
    Loading(bool),
    #[serde(rename = "error")]
    ServerError(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EventType {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "speech")]
    Speech,
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "error")]
    ServerError,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WsRequest {
    pub from: u64,
    pub to: u64,
    pub event: Event,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub reply_msg_id: Option<String>,
}

pub type  WsResponse = WsRequest;
