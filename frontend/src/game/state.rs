use super::{Card, CompletedTrick, Hand, Rank, Suit, Trick};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

impl Seat {
    pub const ALL: [Seat; 4] =
        [Seat::North, Seat::East, Seat::South, Seat::West];

    pub fn label(self) -> &'static str {
        match self {
            Seat::North => "North",
            Seat::East => "East",
            Seat::South => "South",
            Seat::West => "West",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Seat::North => "N",
            Seat::East => "E",
            Seat::South => "S",
            Seat::West => "W",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn partner(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn is_same_partnership(self, other: Self) -> bool {
        self == other || self.partner() == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Turn {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Lobby,
    Dealing,
    Bidding,
    Playing,
    HandComplete,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BidRecord {
    pub player_id: String,
    pub bid: super::commands::Bid,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BiddingState {
    pub bids: Vec<BidRecord>,
    pub consecutive_passes: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contract {
    pub level: u8,
    pub strain: super::commands::BidStrain,
    pub multiplier: u8,
}

impl Contract {
    pub fn label(&self) -> String {
        format!(
            "{}{}{}",
            self.level,
            self.strain.symbol(),
            match self.multiplier {
                1 => "",
                2 => "X",
                4 => "XX",
                _ => "",
            }
        )
    }
}

impl BiddingState {
    pub fn is_complete(&self, has_contract: bool) -> bool {
        if has_contract {
            self.consecutive_passes >= 3
        } else {
            self.consecutive_passes >= 4
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreState {
    pub north_south: u32,
    pub east_west: u32,
}

impl Turn {
    pub fn seat(self) -> Seat {
        match self {
            Turn::North => Seat::North,
            Turn::East => Seat::East,
            Turn::South => Seat::South,
            Turn::West => Seat::West,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub seat: Seat,
    pub connected: bool,
    pub ready: bool,
    pub card_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayedCard {
    pub player_id: String,
    pub seat: Seat,
    pub card: Card,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub version: u64,
    pub room_id: String,
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub my_seat: Seat,
    pub my_hand: Hand,
    pub current_turn: Turn,
    pub bidding: BiddingState,
    pub current_trick: Trick,
    pub completed_tricks: Vec<CompletedTrick>,
    pub last_trick_winner: Option<Seat>,
    pub contract: Option<Contract>,
    pub trump: Option<Suit>,
    pub declarer: Option<Seat>,
    pub tricks_won: u8,
    pub tricks_required: u8,
    pub score: ScoreState,
}

impl GameState {
    pub fn local_room(
        room_id: impl Into<String>,
        player_name: impl Into<String>,
        my_seat: Seat,
    ) -> Self {
        let player_name = player_name.into();
        Self {
            version: 0,
            room_id: room_id.into(),
            phase: GamePhase::Lobby,
            players: Seat::ALL
                .into_iter()
                .map(|seat| Player {
                    id: local_player_id(seat),
                    name: if seat == my_seat {
                        player_name.clone()
                    } else {
                        seat.label().to_string()
                    },
                    seat,
                    connected: true,
                    ready: true,
                    card_count: 0,
                })
                .collect(),
            my_seat,
            my_hand: Hand::default(),
            current_turn: Turn::North,
            bidding: BiddingState::default(),
            current_trick: Trick::new(Seat::North),
            completed_tricks: Vec::new(),
            last_trick_winner: None,
            contract: None,
            trump: None,
            declarer: None,
            tricks_won: 0,
            tricks_required: 0,
            score: ScoreState::default(),
        }
    }

    pub fn demo() -> Self {
        Self {
            version: 0,

            room_id: "DEMO".into(),
            phase: GamePhase::Playing,

            players: vec![
                Player {
                    id: "north".into(),
                    name: "North".into(),
                    seat: Seat::North,
                    connected: true,
                    ready: true,
                    card_count: 13,
                },
                Player {
                    id: "east".into(),
                    name: "East".into(),
                    seat: Seat::East,
                    connected: true,
                    ready: true,
                    card_count: 13,
                },
                Player {
                    id: "south".into(),
                    name: "You".into(),
                    seat: Seat::South,
                    connected: true,
                    ready: true,
                    card_count: 13,
                },
                Player {
                    id: "west".into(),
                    name: "West".into(),
                    seat: Seat::West,
                    connected: true,
                    ready: true,
                    card_count: 13,
                },
            ],

            my_seat: Seat::South,

            my_hand: Hand::new(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Clubs,
                },
            ]),

            current_turn: Turn::South,
            bidding: BiddingState::default(),

            current_trick: Trick::new(Seat::South),
            completed_tricks: Vec::new(),
            last_trick_winner: None,

            contract: Some(Contract {
                level: 4,
                strain: super::commands::BidStrain::Hearts,
                multiplier: 1,
            }),

            trump: Some(Suit::Hearts),

            declarer: Some(Seat::South),

            tricks_won: 3,

            tricks_required: 10,
            score: ScoreState::default(),
        }
    }

    pub fn current_player(&self) -> Option<&Player> {
        let seat = self.current_turn.seat();

        self.players.iter().find(|player| player.seat == seat)
    }

    pub fn is_my_turn(&self) -> bool {
        self.current_turn.seat() == self.my_seat
    }

    /// UX helper only. The server remains authoritative for legal card play.
    pub fn can_play_card(&self, card: Card) -> bool {
        self.is_my_turn() && self.can_follow_suit(card)
    }

    pub fn can_follow_suit(&self, card: Card) -> bool {
        if !self.my_hand.contains(&card) {
            return false;
        }
        self.current_trick.led_suit().is_none_or(|led| {
            card.suit == led
                || !self.my_hand.iter().any(|held| held.suit == led)
        })
    }

    pub fn player_at(&self, seat: Seat) -> Option<&Player> {
        self.players.iter().find(|player| player.seat == seat)
    }

    pub fn player_at_mut(&mut self, seat: Seat) -> Option<&mut Player> {
        self.players.iter_mut().find(|player| player.seat == seat)
    }
}

pub fn local_player_id(seat: Seat) -> String {
    seat.label().to_lowercase()
}
