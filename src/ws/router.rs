use std::{net::SocketAddr, ops::ControlFlow, sync::Arc};

use crate::utils::event;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Query, State, WebSocketUpgrade,
    },
    headers,
    response::IntoResponse,
    routing::get,
    Router, TypedHeader,
};
use flume::{unbounded, Sender};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use super::state::WsState;

pub fn router(state: Arc<WsState>) -> Router {
    Router::new().route("/", get(ws_handler)).with_state(state)
}

#[derive(Deserialize)]
pub struct SubjectArgs {
    pub uid: u64,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
    Query(SubjectArgs { uid }): Query<SubjectArgs>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("user {} {} connected from {}", uid, user_agent, addr);
    let uuid = Arc::new(Uuid::new_v4());
    // insert_user_uuid_map(user_uuid_map, uid, uuid.clone());
    state.insert_user_uuid_map(uid, uuid.clone());
    ws.on_upgrade(move |socket| handle_socket(state.clone(), uuid, socket, addr))
}

async fn handle_socket(state: Arc<WsState>, uuid: Arc<Uuid>, socket: WebSocket, who: SocketAddr) {
    let (s1, r1) = unbounded::<Arc<event::WsRequest>>();
    let (mut sender, mut receiver) = socket.split();
    state.insert_user_peer_map(uuid.clone(), s1);
    tokio::spawn(async move {
        while let Ok(msg) = r1.recv() {
            sender
                .send(Message::Text(
                    serde_json::to_string(&msg.clone().as_ref()).unwrap(),
                ))
                .await
                .unwrap();
        }
    });
    tokio::spawn(async move {
        let state = state.clone();
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            if process_message(state.sender.clone(), uuid.clone(), msg, who).is_break() {
                break;
            }
        }
        cnt
    });
}

fn process_message(
    s: Sender<event::ChannelMessage>,
    uuid: Arc<Uuid>,
    msg: Message,
    who: SocketAddr,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => match serde_json::from_str::<event::WsRequest>(&t) {
            Ok(mut msg) => {
                info!(" {} sent message: {:?}", who, msg);
                s.send(event::ChannelMessage { uuid, body: msg })
                    .map_err(|e| {
                        info!(" {} sent message error: {:#?}", who, e.to_string());
                    })
                    .unwrap();
            }
            Err(e) => {
                info!(" {} sent unknown message: {:?}", who, t);
            }
        },
        Message::Binary(d) => {
            info!(" {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                info!(
                    "{} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                info!(" {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            info!(" {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            info!(" {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
