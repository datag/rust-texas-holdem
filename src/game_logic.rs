// https://en.wikipedia.org/wiki/Texas_hold_%27em#Play_of_the_hand
// https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
// https://de.wikipedia.org/wiki/Texas_Hold%E2%80%99em#Wahrscheinlichkeiten

use std::{collections::HashMap, fmt};

use crate::card::{Card, Face, Suit};

const HAND_COMBINATIONS: u32 = 133_784_560;       // (52 !) / ((45 !) * (7 !))

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    face_map: HashMap<Face, Vec<Card>>,
    face_count: Vec<FaceCount>,
}

// FIXME: Rename; hand should be final hand, but the strength calculation with all cards
// TODO: Final hand should be cards 2-5 and the strength
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


        // Build a hash map with Face as key and matching Cards as value
        let mut face_map: HashMap<Face, Vec<Card>> = HashMap::new();
        for card in &cards {
            let cards_of_face = face_map.entry(card.face).or_default();
            cards_of_face.push(*card);
        }

        
        // Build a vector of tuples (card_count, Face)
        let mut face_count: Vec<FaceCount> = face_map
            .iter()
            .map(|(face, cards)| FaceCount {face: *face, count: cards.len()})
            .collect();

        // sort by count and after that by face
        face_count.sort_by(|a, b| {
            b.count.cmp(&a.count).then_with(|| b.face.cmp(&a.face))
        });


        Self {
            cards,
            face_map,
            face_count,
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
        if let Some(strength) = self.eval_flush() {
            let match_suit = strength.rank_cards.unwrap().first().unwrap().suit;

            if let Some(strength) = self.eval_straight(Some(match_suit)) {
                let cards = &strength.rank_cards.unwrap();
                let top_card = cards.first().unwrap();
                if top_card.face == Face::Ace {
                    Some(Strength {
                        ranking: Ranking::RoyalFlush,
                        rank_cards: None,
                        kicker_cards: None,
                    })
                } else {
                    Some(Strength {
                        ranking: Ranking::StraightFlush,
                        rank_cards: Some(vec![*top_card]),
                        kicker_cards: None,
                    })
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eval_four_of_a_kind(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(4)
    }

    fn eval_full_house(&self) -> Option<Strength> {
        let trips_slot = self.face_count.first()?;
        let pair_slot = self.face_count.get(1)?;
        
        if trips_slot.count >= 3 && pair_slot.count >= 2 {
            Some(Strength {
                ranking: Ranking::FullHouse,
                rank_cards: Some(vec![
                    self.face_map.get(&trips_slot.face).unwrap()[0],     // trips card
                    self.face_map.get(&pair_slot.face).unwrap()[0],      // pair card
                ]),
                kicker_cards: None,
            })
        } else {
            None
        }
    }

    fn eval_flush(&self) -> Option<Strength> {
        let mut suit_card_map: HashMap<Suit, Vec<Card>> = HashMap::new();

        for card in &self.cards {
            let cards_of_suit = suit_card_map.entry(card.suit).or_default();
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
        let mut consecutive = 1;
        let mut prev_card = &self.cards[0];
        let mut top_card = &self.cards[0];
        let mut is_straight = false;

        for card in &self.cards[1..] {
            if card.face as u8 == (prev_card.face as u8) - 1 && match_suit.map(|suit| card.suit == suit).unwrap_or(true) {
                consecutive += 1;
            } else {
                consecutive = 1;
                top_card = card;
            }

            if consecutive == 5 {
                is_straight = true;
                break;
            }

            prev_card = card;
        }

        // Test for special case 5432A straight ("wheel")
        if !is_straight && consecutive == 4 && top_card.face == Face::Five
            && self.cards[0].face == Face::Ace && match_suit.map(|suit| self.cards[0].suit == suit).unwrap_or(true) {
            is_straight = true;
        }

        if is_straight {
            Some(Strength {
                ranking: Ranking::Straight,
                rank_cards: Some(vec![*top_card]),
                kicker_cards: None,
            })
        } else {
            None
        }
    }

    fn eval_three_of_a_kind(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(3)
    }

    fn eval_two_pair(&self) -> Option<Strength> {
        let first_pair_slot = self.face_count.first()?;
        let second_pair_slot = self.face_count.get(1)?;
        
        if first_pair_slot.count >= 2 && second_pair_slot.count >= 2 {
            let kicker_slot = self.face_count.get(2);

            Some(Strength {
                ranking: Ranking::TwoPair,
                rank_cards: Some(vec![
                    self.face_map.get(&first_pair_slot.face).unwrap()[0],     // first pair card
                    self.face_map.get(&second_pair_slot.face).unwrap()[0],    // second pair card
                ]),
                kicker_cards: kicker_slot.map(|kicker_slot| vec![self.face_map.get(&kicker_slot.face).unwrap()[0]]),
            })
        } else {
            None
        }
    }

    fn eval_one_pair(&self) -> Option<Strength> {
        self.eval_n_of_a_kind(2)
    }

    fn eval_n_of_a_kind(&self, n: usize) -> Option<Strength> {
        let n_of_a_kind_slot = self.face_count.first()?;
        
        if n_of_a_kind_slot.count >= n {
            let kicker_count = 4 - n + 1;
            // filter for cards other than the n-of-a-kind face and take remaining cards for kicker
            let kicker_cards: Vec<Card> = self.cards.iter().filter(|&c| c.face != n_of_a_kind_slot.face).take(kicker_count).cloned().collect();

            Some(Strength {
                ranking: match n {
                    2 => Ranking::OnePair,
                    3 => Ranking::ThreeOfAKind,
                    4 => Ranking::FourOfAKind,
                    _ => unreachable!(),
                },
                rank_cards: Some(vec![
                    self.face_map.get(&n_of_a_kind_slot.face).unwrap()[0],     // n-of-a-kind card
                ]),
                kicker_cards: match !kicker_cards.is_empty() {
                    true => Some(kicker_cards),
                    false => None,
                },
            })
        } else {
            None
        }
    }

    fn eval_high_card(&self) -> Strength {
        Strength {
            ranking: Ranking::HighCard,
            rank_cards: Some(vec![self.cards[0]]),                                      // top card
            kicker_cards: Some(self.cards.iter().skip(1).take(4).cloned().collect()),   // remaining 4
        }
    }
}

#[derive(Debug)]
struct FaceCount {
    pub face: Face,
    pub count: usize,
}

#[derive(Debug)]
pub struct Strength {
    pub ranking: Ranking,
    pub rank_cards: Option<Vec<Card>>,
    pub kicker_cards: Option<Vec<Card>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
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
