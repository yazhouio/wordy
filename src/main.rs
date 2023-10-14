#![feature(async_closure)]

use std::net::SocketAddr;
use std::sync::{Arc};
use std::{path::PathBuf};

use anyhow::{anyhow, Result};

use axum::Router;
use flume::{unbounded, Receiver, Sender};

use tower_http::services::ServeDir;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tracing::{info, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use utils::event;

use crate::utils::azure_tts::fetch_speed;
use crate::utils::event::WsResponse;

mod utils;
mod ws;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "chat_ws=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let (s, r) = unbounded::<event::ChannelMessage>();
    let state = Arc::new(ws::state::WsState::new(s.clone()));

    tokio::spawn(handle_message(r, state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr.to_string());
    let app = Router::new()
    .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
    .nest("/ws", ws::router::router(state.clone()))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true)),
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn handle_message(r: Receiver<event::ChannelMessage>, state: Arc<ws::state::WsState>) {
    loop {
        while let Ok(msg) = r.recv_async().await {
            // println!("handle_message: {:?}", msg);
            handle_message_item(msg, state.clone()).await;
        }
    }
}

// fn get_uuid_by_uid(user_uuid_map: ws::UserUUidMap, to: u64) -> Option<Vec<Arc<Uuid>>> {
//     let user_uuid_map = user_uuid_map.lock().unwrap();
//     if !user_uuid_map.contains_key(&to) {
//         return None;
//     }
//     // user_uuid_map.get(&to).iter().cloned().flatten(||).cloned().collect()
// }

// fn get_sender_by_uuid(
//     user_peer_map: ws::UserPeerMap,
//     uuid: Arc<Uuid>,
// ) -> Option<Sender<Arc<event::WsRequest>>> {
//     let user_peer_map = user_peer_map.lock().unwrap();
//     if !user_peer_map.contains_key(&uuid) {
//         return None;
//     }
//     let sender = user_peer_map.get(&uuid).unwrap();
//     Some(sender.clone())
// }

async fn handle_message_item(
    msg: event::ChannelMessage,
    state: Arc<ws::state::WsState>,
) -> Result<()> {
    let msg_id = Arc::new(Uuid::new_v4().to_string());
    info!("handle_message_item: {:?}", msg.body.event);

    let uuids = if msg.body.to == 0 {
        Some(vec![msg.uuid.clone()])
    } else {
        state.get_user_uuid_map(msg.body.to)
    };

    if uuids.is_none() {
        info!("user {} not found", msg.body.to.to_string());
        return Ok(());
    }

    for uuid in uuids.unwrap() {
        let sender = state.get_user_peer_map(uuid.clone());
        if sender.is_none() {
            info!("user {} not found", msg.body.to.to_string());
            continue;
        }
        let sender = sender.unwrap();
        let msg_id = msg_id.clone();
        let msg = Arc::new(msg.body.clone());
        tokio::spawn(async move {
            let msg = msg.clone();
            match msg.to {
                0 => {
                    handle_system_message(msg, msg_id.clone(), &sender)
                        .await
                        .map_err(|e| {
                            tracing::error!("handle_system_message error: {:?}", e);
                            anyhow!("handle_system_message error: {:?}", e)
                        })
                        .unwrap();
                }
                _ => {
                    sender.send(msg).unwrap();
                }
            };
            // Ok(())
        });
    }
    Ok(())
}

async fn handle_system_message(
    msg: Arc<event::WsRequest>,
    msg_id: Arc<String>,
    sender: &Sender<Arc<event::WsRequest>>,
) -> Result<()> {
    let resp = WsResponse {
        event: event::Event::Loading(true),
        event_type: event::EventType::Loading,
        msg_id: msg_id.clone().to_string(),
        from: 0,
        to: msg.from,
        reply_msg_id: Some(msg.msg_id.clone()),
    };
    sender.send(Arc::new(resp)).unwrap();
    let sender = sender.clone();
    let msg_id = msg_id.clone().to_string();
    tokio::spawn(async move {
        let resp = handle_system_message_item(msg, msg_id.clone().to_string())
            .await
            .map_err(|e| {
                tracing::error!("handle_system_message_item error: {:?}", e);
                anyhow!(e)
            })
            .unwrap();
        println!("{:#?}", resp);
        sender.clone().send(Arc::new(resp)).unwrap();
    });
    Ok(())
}

async fn handle_system_message_item(
    msg: Arc<event::WsRequest>,
    msg_id: String,
) -> Result<event::WsResponse> {
    let msg = msg.clone();
    let mut resp = WsResponse {
        event: msg.event.clone(),
        event_type: event::EventType::Loading,
        msg_id,
        from: 0,
        to: msg.from,
        reply_msg_id: None,
    };
    match msg.event.clone() {
        event::Event::Chat(message) => {
            let openai_key = std::env::var("OPENAI_API_KEY").unwrap();
            // let text = en_teacher_chat(&openai_key, &message).await?;
            // let res = text.choices[0].message.content.to_owned();
            let res = "天空的英文是`sky`。它是指地球上大气层上方的空间，通常是呈现蓝色或灰色的。这是它的英文例句：1. `The sky is so clear today, not a single cloud in sight.` 2. `When the sun sets, the sky turns into a beautiful mixture of pink, purple, and orange colors.`".to_owned();
            resp.event = event::Event::Chat(res);
            resp.event_type = event::EventType::Chat;
        }
        event::Event::Speech(message) => {
            let azure_tts_key = std::env::var("AZURE_TTS_KEY").unwrap();
            let region = std::env::var("AZURE_TTS_REGION").unwrap();
            let path = fetch_speed(&azure_tts_key, &region, &message).await?;
            resp.event = event::Event::Speech(path);
            resp.event_type = event::EventType::Speech;
            resp.reply_msg_id = Some(msg.msg_id.clone());
        }
        _ => {
            return Err(anyhow::anyhow!("unknown event type"));
        }
    };
    Ok(resp)
}
