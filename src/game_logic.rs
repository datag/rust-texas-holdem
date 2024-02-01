// https://en.wikipedia.org/wiki/Texas_hold_%27em#Play_of_the_hand
// https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
// https://de.wikipedia.org/wiki/Texas_Hold%E2%80%99em#Wahrscheinlichkeiten

use std::fmt;

const HAND_COMBINATIONS: u32 = 133_784_560;       // (52 !) / ((45 !) * (7 !))

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
