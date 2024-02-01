use std::fmt;

use rand::{seq::SliceRandom, thread_rng};

use crate::card::{Card, Face, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for face in [
                Face::Ace,
                Face::King,
                Face::Queen,
                Face::Jack,
                Face::Ten,
                Face::Nine,
                Face::Eight,
                Face::Seven,
                Face::Six,
                Face::Five,
                Face::Four,
                Face::Three,
                Face::Two,
            ] {
                cards.push(Card { face, suit });
            }
        }

        Self { cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.cards.iter()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_names: Vec<String> = self.cards.iter().map(|c| format!("{c}")).collect();

        f.debug_struct("Deck").field("cards", &card_names).field("len", &self.cards.len()).finish()
    }
}
