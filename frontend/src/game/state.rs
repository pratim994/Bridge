use super::{Card, Rank, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

impl Seat {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    North,
    East,
    South,
    West,
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
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub seat: Seat,
    pub connected: bool,
    pub card_count: usize,
}

#[derive(Debug, Clone)]
pub struct PlayedCard {
    pub player_id: String,
    pub card: Card,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub version: u64,
    pub room_id: String,
    pub players: Vec<Player>,
    pub my_seat: Seat,
    pub my_hand: Vec<Card>,
    pub current_turn: Turn,
    pub current_trick: Vec<PlayedCard>,
    pub contract: Option<String>,
    pub trump: Option<Suit>,
    pub declarer: Option<Seat>,
    pub tricks_won: u8,
    pub tricks_required: u8,
}

impl GameState {
    pub fn demo() -> Self {
        Self {
            version: 0,

            room_id: "DEMO".into(),

            players: vec![
                Player {
                    id: "north".into(),
                    name: "North".into(),
                    seat: Seat::North,
                    connected: true,
                },
                Player {
                    id: "east".into(),
                    name: "East".into(),
                    seat: Seat::East,
                    connected: true,
                },
                Player {
                    id: "south".into(),
                    name: "You".into(),
                    seat: Seat::South,
                    connected: true,
                },
                Player {
                    id: "west".into(),
                    name: "West".into(),
                    seat: Seat::West,
                    connected: true,
                },
            ],

            my_seat: Seat::South,

            my_hand: vec![
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
            ],

            current_turn: Turn::South,

            current_trick: Vec::new(),

            contract: Some("4♥".into()),

            trump: Some(Suit::Hearts),

            declarer: Some(Seat::South),

            tricks_won: 3,

            tricks_required: 10,
        }
    }

    pub fn current_player(&self) -> Option<&Player> {
        let seat = self.current_turn.seat();

        self.players
            .iter()
            .find(|player| player.seat == seat)
    }

    pub fn is_my_turn(&self) -> bool {
        self.current_turn.seat() == self.my_seat
    }

    pub fn player_at(&self, seat: Seat) -> Option<&Player> {
        self.players
            .iter()
            .find(|player| player.seat == seat)
    }
}