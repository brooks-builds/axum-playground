use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/mirror", post(mirror_mirror));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("Listening on {address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello from root"
}

async fn mirror_mirror(Json(payload): Json<MirrorOnTheWall>) -> impl IntoResponse {
    (StatusCode::OK, Json(payload))
}

#[derive(Deserialize, Serialize)]
struct MirrorOnTheWall {
    pub message: String,
}
