use super::{Card, Hand, Rank, Suit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn standard() -> Self {
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        Self {
            cards: suits
                .into_iter()
                .flat_map(|suit| ranks.map(|rank| Card { rank, suit }))
                .collect(),
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Shuffles with a seeded xorshift generator, making local deals reproducible.
    pub fn shuffled(mut self, seed: u64) -> Self {
        let mut state = if seed == 0 {
            0x9e37_79b9_7f4a_7c15
        } else {
            seed
        };
        for index in (1..self.cards.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let other = (state as usize) % (index + 1);
            self.cards.swap(index, other);
        }
        self
    }

    /// Deals round-robin beginning with North. The returned array is indexed by `Seat`.
    pub fn deal(self) -> [Hand; 4] {
        let mut hands: [Hand; 4] = std::array::from_fn(|_| Hand::default());
        for (index, card) in self.cards.into_iter().enumerate() {
            hands[index % 4].push(card);
        }
        hands
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn standard_deck_has_52_unique_cards() {
        let deck = Deck::standard();
        assert_eq!(deck.cards().len(), 52);
        assert_eq!(
            deck.cards().iter().copied().collect::<HashSet<_>>().len(),
            52
        );
    }

    #[test]
    fn deal_produces_four_hands_of_thirteen_without_duplicates() {
        let hands = Deck::standard().shuffled(42).deal();
        assert!(hands.iter().all(|hand| hand.len() == 13));
        let dealt = hands
            .iter()
            .flat_map(|hand| hand.cards().iter().copied())
            .collect::<HashSet<_>>();
        assert_eq!(dealt.len(), 52);
    }

    #[test]
    fn seeded_shuffle_is_reproducible() {
        assert_eq!(Deck::standard().shuffled(7), Deck::standard().shuffled(7));
    }
}
