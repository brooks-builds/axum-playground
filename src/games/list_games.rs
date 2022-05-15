use axum::{response::IntoResponse, Json};

use crate::api;

pub async fn list_games() -> impl IntoResponse {
    let games = api::list_games().await.unwrap();
    Json(games.games)
}
