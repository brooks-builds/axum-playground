use serde::{Deserialize, Serialize};

pub mod create_game;
pub mod list_games;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
    pub name: String,
}
