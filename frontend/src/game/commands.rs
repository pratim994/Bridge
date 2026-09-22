use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum GameCommand {
    #[serde(rename = "ready")]
    Ready {
        game_id: String,
        command_id: String,
    },

    #[serde(rename = "play_card")]
    PlayCard {
        game_id: String,
        command_id: String,
        card: String,
    },

    #[serde(rename = "leave_game")]
    LeaveGame {
        game_id: String,
        command_id: String,
    },
}