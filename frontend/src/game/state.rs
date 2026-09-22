use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum GamePhase {
    Waiting,
    Bidding,
    Playing,
    Completed,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GameState {
    pub game_id: String,
    pub version: u64,
    pub phase: GamePhase,

    pub current_turn: Option<String>,

    pub hand: Vec<Card>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_id: String::new(),
            version: 0,
            phase: GamePhase::Waiting,
            current_turn: None,
            hand: Vec::new(),
        }
    }
}