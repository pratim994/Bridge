use serde::{Deserialize, Serialize};

use crate::game::{commands::GameCommand, events::GameEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "game_event")]
    GameEvent { event: GameEvent },

    #[serde(rename = "error")]
    Error { message: String },
}

pub type ClientMessage = GameCommand;
