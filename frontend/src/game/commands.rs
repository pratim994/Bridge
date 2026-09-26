use serde::{Deserialize, Serialize};

use super::{card::Card, state::Seat};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum GameCommand {
    #[serde(rename = "join_game")]
    JoinGame {
        room_id: String,
        player_name: String,
    },

    #[serde(rename = "ready")]
    SetReady { ready: bool },

    #[serde(rename = "start_game")]
    StartGame,

    #[serde(rename = "play_card")]
    PlayCard { card: Card },

    #[serde(rename = "place_bid")]
    PlaceBid { bid: Bid },

    #[serde(rename = "leave_game")]
    LeaveGame,

    #[serde(rename = "set_seat")]
    SetLocalSeat { seat: Seat },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bid {
    Pass,
    Contract { level: u8, strain: BidStrain },
    Double,
    Redouble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BidStrain {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    NoTrump,
}

impl BidStrain {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Clubs => "♣",
            Self::Diamonds => "♦",
            Self::Hearts => "♥",
            Self::Spades => "♠",
            Self::NoTrump => "NT",
        }
    }

    pub fn suit(self) -> Option<super::card::Suit> {
        match self {
            Self::Clubs => Some(super::card::Suit::Clubs),
            Self::Diamonds => Some(super::card::Suit::Diamonds),
            Self::Hearts => Some(super::card::Suit::Hearts),
            Self::Spades => Some(super::card::Suit::Spades),
            Self::NoTrump => None,
        }
    }
}

impl Bid {
    pub fn contract(&self) -> Option<(u8, BidStrain)> {
        match self {
            Self::Contract { level, strain } => Some((*level, *strain)),
            Self::Pass | Self::Double | Self::Redouble => None,
        }
    }
}
