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

    SendSignal {
    signal: SignalMessage,
},
SignalReceived {
    signal: SignalMessage,
},
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bid {
    Pass,
    Contract {
        level: u8,
        strain: BidStrain,
    },
    Double,
    Redouble,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BidStrain {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    NoTrump,
}