use serde::{Deserialize, Serialize};

use super::{
    card::Card,
    commands::Bid,
    state::{Player, Seat, Turn},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "game_state")]
    GameState {
        version: u64,
        state: serde_json::Value,
    },

    #[serde(rename = "hand_dealt")]
    HandDealt {
        version: u64,
        hand: Vec<Card>,
        card_counts: [u8; 4],
    },

    #[serde(rename = "card_played")]
    CardPlayed {
        version: u64,
        player_id: String,
        seat: Seat,
        card: Card,
    },

    #[serde(rename = "player_joined")]
    PlayerJoined {
        version: u64,
        player: Player,
    },

    #[serde(rename = "player_left")]
    PlayerLeft {
        version: u64,
        player_id: String,
    },

    PlayerReady {
        version: u64,
        player_id: String,
        ready: bool,
    },

    #[serde(rename = "error")]
    Error {
        message: String,
    },
    BidPlaced {
        version: u64,
        player_id: String,
        bid: Bid,
    },

    TurnChanged {
        version: u64,
        turn: Turn,
    },

    StartTrick {
        version: u64,
        leader: Seat,
    },

    TrickCompleted {
        version: u64,
        winner: Seat,
    },

    ScoreUpdated {
        version: u64,
        north_south: u32,
        east_west: u32,
    },

    GameStarted {
        version: u64,
    },

    GameCompleted {
        version: u64,
    },
    #[serde(rename = "game_finished")]
    GameFinished {
        version: u64,
    },
    ChatMessage {
        player_id: String,
        player_name: String,
        message: String,
    },
}
