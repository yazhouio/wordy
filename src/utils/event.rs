use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EventType {
    Chat,
    Speech,
    Loading,
    ServerError,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MsgEvent {
    pub event: EventType,
    pub body: MsgBody,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MsgRequest {
    pub from: Option<u64>,
    pub msg: String,
    pub to: Option<u64>,
    pub event: EventType,
    pub client_msg_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MsgBody {
    pub from: Option<u64>,
    pub msg: Option<String>,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub to: Option<u64>,
    pub client_msg_id: String,
}
