use serde::{Deserialize, Serialize};

use super::Card;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn cards(&self) -> &[Card] {
        &self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(card)
    }

    pub fn remove(&mut self, card: Card) -> bool {
        if let Some(index) = self.0.iter().position(|held| *held == card) {
            self.0.remove(index);
            true
        } else {
            false
        }
    }

    pub(crate) fn push(&mut self, card: Card) {
        self.0.push(card);
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self::new(cards)
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
