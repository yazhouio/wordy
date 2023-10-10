#![feature(async_closure)]

use std::{net::SocketAddr, path::PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use axum::{response::IntoResponse, Router, routing::get};
use flume::{Receiver, unbounded};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use utils::event;

use crate::utils::azure_tts::fetch_speed;
use crate::utils::event::EventType::Loading;
use crate::utils::event::{WsResponse};
use crate::utils::openai::en_teacher_chat;


mod utils;
mod ws;


#[derive(Deserialize)]
pub struct SubjectArgs {
    pub uid: u64,
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env()
        //         .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        // )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let (s, r) = unbounded::<event::WsRequest>();
    let user_peer_map: ws::UserPeerMap = Arc::new(Mutex::new(HashMap::new()));
    tokio::spawn(
        handle_message(r, user_peer_map.clone())
    );
    // build our application with some routes
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .nest("/ws", Router::new().route("/", get(ws::ws_handler))
            .with_state((s, user_peer_map.clone())),
        )
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let addr = "127.0.0.1:3000";
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(
            app.into_make_service_with_connect_info::<SocketAddr>()
        )
        .await
        .unwrap();
}

async fn handle_message(r: Receiver<event::WsRequest>, user_peer_map: ws::UserPeerMap) {
    loop {
        while let Ok(msg) = r.recv_async().await {
            // println!("handle_message: {:?}", msg);
            handle_message_item(msg, user_peer_map.clone()).await;
        }
    }
}

async fn handle_message_item(msg: event::WsRequest, user_peer_map: ws::UserPeerMap) -> Result<()> {
    let uuid = Uuid::new_v4().to_string();
    tracing::info!( "handle_message_item: {:?}", msg);
    match &msg.to {
        0 => {
            handle_system_message(msg, &uuid, user_peer_map.clone()).await?;
            return Ok(());
        },
        to => {
            let mut user_peer_map = user_peer_map.lock().unwrap();
            if !user_peer_map.contains_key(to) {
                println!("user {} not found", to);
                return Ok(());
            }
            let sender = user_peer_map.get_mut(to).unwrap();
            sender.send(msg).unwrap();
        }

    }
    Ok(())
}

async fn handle_system_message(msg: event::WsRequest, msg_id: &str, user_peer_map1: ws::UserPeerMap) -> Result<()> {
    let mut user_peer_map = user_peer_map1.lock().unwrap();
    if  msg.to == 0 {
        let sender = user_peer_map.get_mut(&msg.from).unwrap();
        let resp = WsResponse {
            event: event::Event::Loading(true),
            event_type: event::EventType::Loading,
            msg_id: msg_id.to_owned(),
            from: 0,
            to: msg.from,
            reply_msg_id: Some(msg.msg_id.clone()),
        };
        sender.send(resp.clone()).unwrap();
        let sender = sender.clone();
        let msg_id = msg_id.clone().to_string();
        tokio::spawn(async move {
            let resp = handle_system_message_item(msg, msg_id.clone().to_string()).await.map_err(|e| {
                tracing::error!("handle_system_message_item error: {:?}", e);
                anyhow!(e)
            }).unwrap();
            sender.clone().send(resp).unwrap();
        });
    }
    Ok(())
}


async fn handle_system_message_item(msg: event::WsRequest, msg_id: String) -> Result<event::WsResponse> {
    let mut resp = WsResponse {
        event: msg.event.clone(),
        event_type: event::EventType::Loading,
        msg_id,
        from: 0,
        to: msg.from,
        reply_msg_id: None,
    };
    match msg.event {
        event::Event::Chat(message) => {
            let openai_key = std::env::var("OPENAI_API_KEY").unwrap();
            let text = en_teacher_chat(&openai_key, &message).await?;
            let res = text.choices[0].message.content.to_owned();
            // let res = "天空的英文是`sky`。它是指地球上大气层上方的空间，通常是呈现蓝色或灰色的。这是它的英文例句：1. `The sky is so clear today, not a single cloud in sight.` 2. `When the sun sets, the sky turns into a beautiful mixture of pink, purple, and orange colors.`".to_owned();
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

