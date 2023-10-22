use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::sync::mpsc;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::event;

use crate::channel::handle_message;

mod auth;
mod channel;
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
    let (s, mut r) = mpsc::unbounded_channel::<event::ChannelMessage>();
    let state = Arc::new(ws::state::WsState::new(s.clone()));
    let state1 = state.clone();
    tokio::spawn(async move {
        handle_message(&mut r, state1).await;
    });

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
        .route("/api/:user/salt", get(handle_client_salt))
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

async fn handle_login(req: axum::Json<auth::LoginRequest>) -> impl IntoResponse {
    info!("login request: {:?}", req);
    let db_user = auth::Auth::new_by_name(req.name.to_owned());
    if db_user.is_err() {
        return (
            StatusCode::UNAUTHORIZED,
            "Unauthorized: invalid username or password",
        )
            .into_response();
    }
    let resp = db_user.unwrap().login(&req.name, &req.password);
    match resp {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            "Unauthorized: invalid username or password",
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
struct ClientSaltRequest {
    salt: String,
}

async fn handle_client_salt(Path(name): Path<String>) -> Json<ClientSaltRequest> {
    info!("client salt request: {:?}", name);
    let salt = match auth::Auth::new_by_name(name.to_owned()) {
        Ok(db_user) => db_user.client_salt,
        Err(_) => {
            let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            rand_string
        }
    };
    Json(ClientSaltRequest { salt })
}
