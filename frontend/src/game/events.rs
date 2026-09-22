use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "game_state")]
    GameState {
        version: u64,
        state: serde_json::Value,
    },

    #[serde(rename = "card_played")]
    CardPlayed {
        version: u64,
        player_id: String,
        card: String,
    },

    #[serde(rename = "player_joined")]
    PlayerJoined {
        version: u64,
        player_id: String,
    },

    #[serde(rename = "player_left")]
    PlayerLeft {
        version: u64,
        player_id: String,
    },

    #[serde(rename = "error")]
    Error {
        message: String,
    },
}