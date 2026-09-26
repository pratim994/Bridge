use serde::{Deserialize, Serialize};

use super::{Seat, Suit, state::PlayedCard};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trick {
    pub leader: Seat,
    pub plays: Vec<PlayedCard>,
}

impl Trick {
    pub fn new(leader: Seat) -> Self {
        Self {
            leader,
            plays: Vec::with_capacity(4),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.plays.len() == 4
    }

    pub fn led_suit(&self) -> Option<Suit> {
        self.plays.first().map(|played| played.card.suit)
    }

    pub fn winner(&self, trump: Option<Suit>) -> Option<Seat> {
        if !self.is_complete() {
            return None;
        }
        let led_suit = self.led_suit()?;
        let winning_suit = match trump {
            Some(trump_suit)
                if self
                    .plays
                    .iter()
                    .any(|play| play.card.suit == trump_suit) =>
            {
                trump_suit
            }
            _ => led_suit,
        };
        self.plays
            .iter()
            .filter(|play| play.card.suit == winning_suit)
            .max_by_key(|play| play.card.rank.strength())
            .map(|play| play.seat)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletedTrick {
    pub trick: Trick,
    pub winner: Seat,
}

impl CompletedTrick {
    pub fn new(trick: Trick, trump: Option<Suit>) -> Option<Self> {
        if !trick.is_complete() {
            return None;
        }
        let winner = trick.winner(trump)?;
        Some(Self { trick, winner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Rank, card::Card};

    fn played(seat: Seat, rank: Rank, suit: Suit) -> PlayedCard {
        PlayedCard {
            player_id: seat.short_label().to_string(),
            seat,
            card: Card { rank, suit },
        }
    }

    #[test]
    fn highest_led_suit_card_wins_without_trump() {
        let mut trick = Trick::new(Seat::North);
        trick.plays = vec![
            played(Seat::North, Rank::King, Suit::Hearts),
            played(Seat::East, Rank::Ace, Suit::Clubs),
            played(Seat::South, Rank::Two, Suit::Hearts),
            played(Seat::West, Rank::Ace, Suit::Hearts),
        ];
        assert_eq!(trick.winner(None), Some(Seat::West));
    }

    #[test]
    fn highest_trump_beats_led_suit() {
        let mut trick = Trick::new(Seat::North);
        trick.plays = vec![
            played(Seat::North, Rank::Ace, Suit::Hearts),
            played(Seat::East, Rank::Two, Suit::Spades),
            played(Seat::South, Rank::King, Suit::Spades),
            played(Seat::West, Rank::Three, Suit::Diamonds),
        ];
        assert_eq!(trick.winner(Some(Suit::Spades)), Some(Seat::South));
    }

    #[test]
    fn incomplete_trick_has_no_winner() {
        let mut trick = Trick::new(Seat::North);
        trick
            .plays
            .push(played(Seat::North, Rank::Ace, Suit::Hearts));
        assert_eq!(trick.winner(None), None);
        assert!(CompletedTrick::new(trick, None).is_none());
    }
}
