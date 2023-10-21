use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WsRequest {
    pub from: u64,
    pub to: u64,
    pub event: Event,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "replyMsgId")]
    pub reply_msg_id: Option<String>,
}

#[derive(Debug)]
pub struct ChannelMessage {
    pub uuid: Arc<Uuid>,
    pub body: WsRequest,
}

pub type WsResponse = WsRequest;
