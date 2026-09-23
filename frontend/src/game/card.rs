use serde::{Deserialize, Serialize};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Clubs => "♣",
            Self::Diamonds => "♦",
            Self::Hearts => "♥",
            Self::Spades => "♠",
        }
    }

    pub fn filename(self) -> &'static str {
        match self {
            Self::Clubs => "clubs",
            Self::Diamonds => "diamonds",
            Self::Hearts => "hearts",
            Self::Spades => "spades",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn filename(self) -> &'static str {
        match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "jack",
            Self::Queen => "queen",
            Self::King => "king",
            Self::Ace => "ace",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn asset_path(self) -> String {
        format!(
            "/components/cards/{}_of_{}.png",
            self.rank.filename(),
            self.suit.filename()
        )
    }
}

#[component]
pub fn CardBack() -> impl IntoView {
    view! {
        <div class="card-back">
            <img
                src="/components/cards/card_back.png"
                alt="Card back"
                draggable="false"
            />
        </div>
    }
}