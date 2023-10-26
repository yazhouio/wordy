use std::{net::SocketAddr, ops::ControlFlow, sync::Arc, collections::HashMap};

use anyhow::anyhow;
use axum::{
    extract::{
        ws::{Message, WebSocket, rejection::{WebSocketUpgradeRejection, self}, CloseFrame},
        ConnectInfo, Query, State, WebSocketUpgrade,
    },
    headers,
    http::{StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get},
    Router, TypedHeader, body::Empty,
};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{Algorithm, Validation};
use serde::Deserialize;
// use flume::{unbounded, Sender};
use tokio::{sync::mpsc};
use tracing::{debug, info};
use uuid::Uuid;

use super::state::WsState;
use crate::{
    auth::{jwt, JWTData},
    utils::event,
};

pub fn router(state: Arc<WsState>) -> Router {
    Router::new()
        .route("/", get(ws_handler))
        .with_state(state)
        .fallback(any(handler404))
}

pub async fn handler404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

type Sender<T> = mpsc::UnboundedSender<T>;
#[derive(Deserialize)]
pub struct SubjectArgs {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

pub fn insert(state: Arc<WsState>, uid: u64, uuid: Arc<Uuid>) {
    state.insert_user_uuid_map(uid, uuid.clone());
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
    Query(query): Query<HashMap<String, String>>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("ws_handler query: {:?}", query);
    if (!query.contains_key("accessToken")) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    let access_token = query.get("accessToken").unwrap();
    debug!("ws_handler token: {}", access_token);
    let user = jsonwebtoken::decode::<JWTData>(
        &access_token,
        &jwt::KEYS.decoding,
        &Validation::new(Algorithm::HS256),
    );
    if user.is_err() {
        info!("user {} unauthorized", addr);
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };
    let uid = user.unwrap().claims.id;
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("user {} {} connected from {}", uid, user_agent, addr);
    let uuid = Arc::new(Uuid::new_v4());
    ws.on_upgrade(move |socket| handle_socket(state.clone(), uid, uuid, socket, addr))
}

fn insert_sender(state: Arc<WsState>, uuid: Arc<Uuid>, sender: Sender<Arc<event::WsRequest>>) {
    state.insert_user_peer_map(uuid.clone(), sender);
}

async fn handle_socket(
    state: Arc<WsState>,
    uid: u64,
    uuid: Arc<Uuid>,
    socket: WebSocket,
    who: SocketAddr,
) {
    // socket.close().await.unwrap();
    let (mut sender, mut receiver) = socket.split();
    sender.send(Message::Close(
        Some(CloseFrame {
            code: 403,
            reason: "Forbidden".into(),
        }),
    )).await.unwrap();
    // socket.close().await.unwrap();
    insert(state.clone(), uid, uuid.clone());
    let (s1, mut r1) = mpsc::unbounded_channel::<Arc<event::WsRequest>>();
    insert_sender(state.clone(), uuid.clone(), s1);
    tokio::spawn(async move {
        while let Some(msg) = r1.recv().await {
            sender
                .send(Message::Text(
                    serde_json::to_string(&msg.clone().as_ref()).unwrap(),
                ))
                .await
                .unwrap();
        }
    });
    let state = state.clone();
    tokio::spawn(async move {
        let state = state.clone();
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(state.sender.clone(), uuid.clone(), msg, who)
                .await
                .is_break()
            {
                break;
            }
        }
    });
}

async fn process_message(
    s: Sender<event::ChannelMessage>,
    uuid: Arc<Uuid>,
    msg: Message,
    who: SocketAddr,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => match serde_json::from_str::<event::WsRequest>(&t) {
            Ok(msg) => {
                info!(" {} sent message: {:?}", who, msg);
                s.send(event::ChannelMessage { uuid, body: msg })
                    .map_err(|e| {
                        info!(" {} sent message error: {:#?}", who, e.to_string());
                    })
                    .unwrap();
            }
            Err(_e) => {
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
