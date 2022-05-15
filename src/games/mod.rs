use serde::{Deserialize, Serialize};

pub mod create_game;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
    pub name: String,
}
