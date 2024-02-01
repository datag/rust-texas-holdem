// https://en.wikipedia.org/wiki/Texas_hold_%27em#Play_of_the_hand
// https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
// https://de.wikipedia.org/wiki/Texas_Hold%E2%80%99em#Wahrscheinlichkeiten

use std::{collections::HashMap, fmt};

use crate::card::{Card, Face, Suit};

const HAND_COMBINATIONS: u32 = 133_784_560;       // (52 !) / ((45 !) * (7 !))

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(hole_cards: (Card, Card), flop_cards: Option<(Card, Card, Card)>, turn_card: Option<Card>, river_card: Option<Card>) -> Self {
        let mut cards = Vec::new();

        cards.push(hole_cards.0);
        cards.push(hole_cards.1);

        if let Some(flop_cards) = flop_cards {
            cards.push(flop_cards.0);
            cards.push(flop_cards.1);
            cards.push(flop_cards.2);
        };

        if let Some(turn_card) = turn_card {
            cards.push(turn_card);
        };

        if let Some(river_card) = river_card {
            cards.push(river_card);
        };

        // Sort cards from high to low
        cards.sort();
        cards.reverse();

        Self {
            cards,
        }
    }

    /*
    	Ranking          Rank card(s)           Kicker card(s)
    	------------------------------------------------------
    	HighCard         Top card               Remaining 4
    	OnePair          Pair card              Remaining 3
    	TwoPair          1st & 2nd Pair card    Remaining 1
    	ThreeOfAKind     Trips card             Remaining 2
    	Straight         Top card               -
    	Flush            Flush cards            -
    	FullHouse        Trips & Pair card      -
    	FourOfAKind      FourOfAKind card       Remaining 1
    	StraightFlush    Top card               -
        RoyalFlush       -                      -
    */
    pub fn strength(&self) -> Strength {
        self.eval_straight_flush()
            .or_else(|| self.eval_four_of_a_kind())
            .or_else(|| self.eval_full_house())
            .or_else(|| self.eval_flush())
            .or_else(|| self.eval_straight(None))
            .or_else(|| self.eval_three_of_a_kind())
            .or_else(|| self.eval_two_pair())
            .or_else(|| self.eval_one_pair())
            .unwrap_or(self.eval_high_card())
    }

    fn eval_straight_flush(&self) -> Option<Strength> {
        // TODO: is flush AND straight w/ matching face
        // TODO: special case royal flush: is straight flush w/ Ace as top card
        None
    }

    fn eval_four_of_a_kind(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(4, None)
    }

    fn eval_full_house(&self) -> Option<Strength> {
        // TODO: use 2x n-of-kind? first for 3, then for 2 w/ face exclude
        None
    }

    fn eval_flush(&self) -> Option<Strength> {
        let mut suit_card_map: HashMap<Suit, Vec<Card>> = HashMap::new();

        for card in &self.cards {
            let suit = card.suit;
            let cards_of_suit = suit_card_map.entry(suit).or_default();
            cards_of_suit.push(*card);

            // We've got 5 of one suit, which is a flush
            if cards_of_suit.len() == 5 {
                return Some(Strength {
                    ranking: Ranking::Flush,
                    rank_cards: Some(cards_of_suit.to_vec()),
                    kicker_cards: None,
                });
            }
        }

        None
    }

    fn eval_straight(&self, match_suit: Option<Suit>) -> Option<Strength> {
        // TODO: special case A2345-straight ("wheel")
        None
    }

    fn eval_three_of_a_kind(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(3, None)
    }

    fn eval_two_pair(&self) -> Option<Strength> {
        // TODO: use 2x n-of-kind? second time w/ face exclude
        None
    }

    fn eval_one_pair(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(2, None)
    }

    fn eval_n_of_a_kind(&self, n: usize, not_face: Option<Face>) -> Option<Strength> {
        None
    }

    fn eval_high_card(&self) -> Strength {
        Strength {
            ranking: Ranking::HighCard,
            rank_cards: None,     // TODO: Top card
            kicker_cards: None,   // TODO: Remaining 4
        }
    }
}

#[derive(Debug)]
pub struct Strength {
    ranking: Ranking,
    rank_cards: Option<Vec<Card>>,
    kicker_cards: Option<Vec<Card>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Ranking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Ranking {
    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            Ranking::HighCard      => "High Card",
            Ranking::OnePair       => "One Pair",
            Ranking::TwoPair       => "Two Pair",
            Ranking::ThreeOfAKind  => "Three Of A Kind",
            Ranking::Straight      => "Straight",
            Ranking::Flush         => "Flush",
            Ranking::FullHouse     => "Full House",
            Ranking::FourOfAKind   => "Four Of A Kind",
            Ranking::StraightFlush => "Straight Flush",
            Ranking::RoyalFlush    => "Royal Flush",
        }
    }

    #[rustfmt::skip]
    pub fn combinations(&self) -> u32 {
        match self {
            Ranking::HighCard      => 23_294_460,
            Ranking::OnePair       => 58_627_800,
            Ranking::TwoPair       => 31_433_400,
            Ranking::ThreeOfAKind  =>  6_461_620,
            Ranking::Straight      =>  6_180_020,
            Ranking::Flush         =>  4_047_644,
            Ranking::FullHouse     =>  3_473_184,
            Ranking::FourOfAKind   =>    224_848,
            Ranking::StraightFlush =>     37_260,
            Ranking::RoyalFlush    =>      4_324,
        }
    }

    pub fn probability(&self) -> f32 {
        self.combinations() as f32 / HAND_COMBINATIONS as f32
    }
}

impl fmt::Display for Ranking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name())
    }
}
