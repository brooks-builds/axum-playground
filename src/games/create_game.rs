use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::api;

use super::CreateGameRequest;

pub async fn create_game(Json(game): Json<CreateGameRequest>) -> impl IntoResponse {
    api::create_game(&game.name).await.unwrap();

    StatusCode::CREATED
}
