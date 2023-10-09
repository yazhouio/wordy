use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::{Arc, Mutex};

use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade}, headers, response::IntoResponse, TypedHeader};
use axum::extract::{Query, State};
use axum::extract::connect_info::ConnectInfo;
use flume::{Sender, unbounded};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;

use crate::utils::event;

pub type UserPeerMap = Arc<Mutex<HashMap<u64, Sender<event::WsResponse>>>>;

#[derive(Deserialize)]
pub struct SubjectArgs {
    pub uid: u64,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State((s, user_peer_map)): State<(Sender<event::WsRequest>, UserPeerMap)>,
    Query(SubjectArgs { uid }): Query<SubjectArgs>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    ws.on_upgrade(move |socket| handle_socket(s, uid, user_peer_map, socket, addr))
}

async fn handle_socket(s: Sender<event::WsRequest>, uid: u64, user_peer_map: UserPeerMap, mut socket: WebSocket, who: SocketAddr) {
    let (s1, r1) = unbounded::<event::WsResponse>();
    let (mut sender, mut receiver) = socket.split();
    user_peer_map.clone().lock().unwrap().insert(uid, s1);
    tokio::spawn(async move {
        while let Ok(msg) = r1.recv() {
            sender.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.unwrap();
        }
    });
    tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            if process_message(s.clone(),  uid, msg, who).is_break() {
                break;
            }
        }
        cnt
    });
}

fn process_message(s: Sender<event::WsRequest>, uid: u64, msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            match serde_json::from_str::<event::WsRequest>(&t) {
                Ok(mut msg) => {
                    println!(">>> {} sent message: {:?}", who,  msg);
                    s.send(msg).map_err(|e| {
                        println!(">>> {} sent message error: {:#?}", who,  e.to_string());
                    }).unwrap();
                },
                Err(e) => {
                    println!(">>> {} sent unknown message: {:?}", who,  t);
                }
            }
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
