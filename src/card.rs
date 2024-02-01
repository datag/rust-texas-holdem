// https://en.wikipedia.org/wiki/French-suited_playing_cards
// https://en.wikipedia.org/wiki/Face_card
// https://en.wikipedia.org/wiki/Playing_card_suit
// https://en.wikipedia.org/wiki/French-suited_playing_cards
// https://en.wikipedia.org/wiki/Playing_cards_in_Unicode

use std::fmt;

#[derive(Debug)]
pub struct Card {
    pub face: Face,
    pub suit: Suit,
}

impl Card {
    pub fn symbol(&self) -> char {
        // Codepoint offset for playing cards
        let mut codepoint: u32 = 0x1F0A0;

        // Offset for suit (0xAx=Spades, 0xBx=Hearts, 0xCx=Diamonds, 0xDx=Clubs)
        codepoint += (4 - (self.suit as u32)) << 4;

        // Offset for face (0x01=Ace, 0x02=Two to 0x0B=Jack, Queen=0x0D, King=0x0E)
        codepoint |= match self.face {
            Face::Ace => 1,
            Face::King | Face::Queen => self.face as u32 + 1, // after face J first comes face C (Knight), after that Q and K
            _ => self.face as u32,
        };

        char::from_u32(codepoint).unwrap()
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.face == other.face
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.face.cmp(&other.face)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", &self.face, &self.suit)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Face {
    Ace   = 14,
    King  = 13,
    Queen = 12,
    Jack  = 11,
    Ten   = 10,
    Nine  =  9,
    Eight =  8,
    Seven =  7,
    Six   =  6,
    Five  =  5,
    Four  =  4,
    Three =  3,
    Two   =  2,
}

impl Face {
    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            Face::Ace   => "Ace",
            Face::King  => "King",
            Face::Queen => "Queen",
            Face::Jack  => "Jack",
            Face::Ten   => "Ten",
            Face::Nine  => "Nine",
            Face::Eight => "Eight",
            Face::Seven => "Seven",
            Face::Six   => "Six",
            Face::Five  => "Five",
            Face::Four  => "Four",
            Face::Three => "Three",
            Face::Two   => "Two",
        }
    }

    #[rustfmt::skip]
    pub fn symbol(&self) -> char {
        match self {
            Face::Ace   => 'A',
            Face::King  => 'K',
            Face::Queen => 'Q',
            Face::Jack  => 'J',
            Face::Ten   => 'T',
            Face::Nine  => '9',
            Face::Eight => '8',
            Face::Seven => '7',
            Face::Six   => '6',
            Face::Five  => '5',
            Face::Four  => '4',
            Face::Three => '3',
            Face::Two   => '2',
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Suit {
    Spades   = 4,
    Hearts   = 3,
    Diamonds = 2,
    Clubs    = 1,
}

impl Suit {
    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            Suit::Spades   => "Spades",
            Suit::Hearts   => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs    => "Clubs",
        }
    }

    #[rustfmt::skip]
    pub fn abbr(&self) -> &str {
        match self {
            Suit::Spades   => "s",
            Suit::Hearts   => "h",
            Suit::Diamonds => "d",
            Suit::Clubs    => "c",
        }
    }

    #[rustfmt::skip]
    pub fn symbol(&self) -> char {
        match self {
            Suit::Spades   => '♠',
            Suit::Hearts   => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs    => '♣',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbr())
    }
}

impl fmt::Debug for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
