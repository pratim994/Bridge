use super::{Card, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub seat: Seat,
}

#[derive(Debug, Clone)]
pub struct PlayedCard {
    pub player_id: String,
    pub card: Card,
}

#[derive(Debug, Clone)]
pub struct GameState {
    /// Server/client state version.
    ///
    /// This will become important once state updates
    /// arrive over WebSocket.
    pub version: u64,

    pub players: Vec<Player>,

    pub my_seat: Seat,

    pub my_hand: Vec<Card>,

    pub current_turn: Turn,

    pub current_trick: Vec<PlayedCard>,

    pub contract: Option<String>,

    pub trump: Option<Suit>,
}

impl GameState {
    pub fn demo() -> Self {
        Self {
            version: 0,

            players: vec![
                Player {
                    id: "north".into(),
                    name: "North".into(),
                    seat: Seat::North,
                },
                Player {
                    id: "east".into(),
                    name: "East".into(),
                    seat: Seat::East,
                },
                Player {
                    id: "south".into(),
                    name: "You".into(),
                    seat: Seat::South,
                },
                Player {
                    id: "west".into(),
                    name: "West".into(),
                    seat: Seat::West,
                },
            ],

            my_seat: Seat::South,

            my_hand: vec![
                Card {
                    rank: super::Rank::Ace,
                    suit: Suit::Spades,
                },
                Card {
                    rank: super::Rank::King,
                    suit: Suit::Spades,
                },
                Card {
                    rank: super::Rank::Queen,
                    suit: Suit::Spades,
                },
                Card {
                    rank: super::Rank::Ten,
                    suit: Suit::Spades,
                },
                Card {
                    rank: super::Rank::Nine,
                    suit: Suit::Spades,
                },
                Card {
                    rank: super::Rank::Ace,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: super::Rank::Jack,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: super::Rank::Seven,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: super::Rank::Ace,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: super::Rank::Eight,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: super::Rank::Four,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: super::Rank::King,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: super::Rank::Three,
                    suit: Suit::Clubs,
                },
            ],

            current_turn: Turn::South,

            current_trick: vec![],

            contract: Some("4♥".into()),

            trump: Some(Suit::Hearts),
        }
    }
}