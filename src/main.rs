#![feature(async_closure)]

use std::net::SocketAddr;
use std::sync::{Arc};
use std::{path::PathBuf};

use anyhow::{anyhow, Result};

use axum::http::{StatusCode, self};
use axum::{Router, Json};
use axum::routing::post;
use flume::{unbounded, Receiver, Sender};

use tower_http::services::ServeDir;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tracing::{info, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};


use utils::event;

use crate::utils::azure_tts::fetch_speed;
use crate::utils::event::WsResponse;
use crate::channel::handle_message;

mod utils;
mod ws;
mod auth;
mod channel;

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

    let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any)
    .allow_headers(Any);

    let app = Router::new()
    .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
    .nest("/ws", ws::router::router(state.clone()))
    .route("/api/login", post(handle_login))
    .layer(cors)
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true)),
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}



async fn handle_login(req: axum::Json<auth::LoginRequest>) -> (StatusCode, Json<auth::LoginResponse>) {
    let db_name = std::env::var("CLIENT_NAME").unwrap();
    let db_password = std::env::var("CLIENT_PASSWORD").unwrap();
    let db_id = std::env::var("CLIENT_ID").unwrap();
    let db_user = auth::Auth::new(db_name, db_id.parse::<u64>().unwrap(), db_password);
     let resp =  db_user.login(&req.name, &req.password);
        match resp {
            Ok(resp) => (StatusCode::OK, Json(resp)),
            Err(_) => (StatusCode::UNAUTHORIZED, Json(auth::LoginResponse {
                name: "".to_string(),
                id: 0,
                token: "".to_string(),
            }))
        }
}
