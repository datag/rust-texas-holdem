use std::{collections::HashMap, io::{self, Write}, time::Instant};

use crate::{
    card::{Card, Face, Suit}, deck::Deck, game_logic::{Hand, Ranking}
};

pub mod card;
pub mod deck;
pub mod game_logic;

fn main() {
    println!("Rust Texas Holdem");

    #[rustfmt::skip]
    let cards = vec![
        Card { face: Face::Ace,   suit: Suit::Spades },
        Card { face: Face::King,  suit: Suit::Hearts },
        Card { face: Face::Queen, suit: Suit::Diamonds },
        Card { face: Face::Jack,  suit: Suit::Clubs },
        Card { face: Face::Ten,   suit: Suit::Hearts },
        Card { face: Face::Two,   suit: Suit::Clubs },
        Card { face: Face::Two,   suit: Suit::Clubs },
    ];

    let ten = Card {
        face: Face::Ten,
        suit: Suit::Spades,
    };

    for card in cards {
        println!(
            "Your card is {card} {} {} ({card:?}) [>Ten {:?}, =Ten {:?}]",
            card.symbol(),
            card.suit.symbol(),
            card > ten,
            card == ten
        );
    }

    println!("----------------------------");

    let mut deck = Deck::new();

    println!("{:?}", deck);

    for card in deck.iter() {
        print!("[{}]", card);
    }
    println!();

    deck.shuffle();

    for card in deck.iter() {
        print!("[{}]", card);
    }
    println!();

    println!("----------------------------");

    let top_card = deck.pop().unwrap();
    println!("Top card: {}", top_card);

    let top_card = deck.pop().unwrap();
    println!("Top card: {}", top_card);

    for ranking in [
        Ranking::HighCard,
        Ranking::OnePair,
        Ranking::TwoPair,
        Ranking::ThreeOfAKind,
        Ranking::Straight,
        Ranking::Flush,
        Ranking::FullHouse,
        Ranking::FourOfAKind,
        Ranking::StraightFlush,
        Ranking::RoyalFlush,
    ] {
        println!("{:>15} ({})\n p = {:>10.6} %", ranking.name(), ranking.combinations(), ranking.probability() * 100f32);
    }

    // println!("----------------------------");

    // let mut deck = Deck::new();

    // deck.shuffle();

    // let hand = Hand::new(
    //     (deck.pop().unwrap(), deck.pop().unwrap()),
    //     Some((deck.pop().unwrap(), deck.pop().unwrap(), deck.pop().unwrap())),
    //     deck.pop(),
    //     deck.pop());

    // println!("{:?}\n{:?}", hand, hand.strength());

    println!("----------------------------");


    // full house
    // let hand = Hand::new(
    //     (Card { face: Face::Jack,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Two,   suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Ace,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Two,   suit: Suit::Diamonds }),
    // );

    // two pairs
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Two,   suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Ace,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Ten,   suit: Suit::Diamonds }),
    // );

    // one pair
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Two,   suit: Suit::Clubs }, Card { face: Face::Nine,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Eight,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Ten,   suit: Suit::Diamonds }),
    // );

    // three of a kind
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Two,   suit: Suit::Clubs }, Card { face: Face::Nine,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Ten,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Ten,   suit: Suit::Diamonds }),
    // );

    // four of a kind
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Ten,   suit: Suit::Clubs }, Card { face: Face::Nine,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Ten,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Ten,   suit: Suit::Diamonds }),
    // );

    // high card
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Two,   suit: Suit::Clubs }, Card { face: Face::Nine,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Eight,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    // );

    // straight
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Queen,   suit: Suit::Clubs }, Card { face: Face::King,  suit: Suit::Clubs }, Card { face: Face::Jack,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Two,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    // );

    // straight wheel
    // let hand = Hand::new(
    //     (Card { face: Face::Five,   suit: Suit::Spades }, Card { face: Face::Four,   suit: Suit::Clubs }),
    //     Some((Card { face: Face::Three,   suit: Suit::Clubs }, Card { face: Face::King,  suit: Suit::Clubs }, Card { face: Face::Ace,  suit: Suit::Hearts })),
    //     Some(Card { face: Face::Two,   suit: Suit::Hearts }),
    //     Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    // );

    // royal flush
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Ace,   suit: Suit::Spades }),
    //     Some((Card { face: Face::Queen,   suit: Suit::Spades }, Card { face: Face::King,  suit: Suit::Spades }, Card { face: Face::Jack,  suit: Suit::Spades })),
    //     Some(Card { face: Face::Two,   suit: Suit::Spades }),
    //     Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    // );

    // straight flush
    // let hand = Hand::new(
    //     (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Nine,   suit: Suit::Spades }),
    //     Some((Card { face: Face::Queen,   suit: Suit::Spades }, Card { face: Face::King,  suit: Suit::Spades }, Card { face: Face::Jack,  suit: Suit::Spades })),
    //     Some(Card { face: Face::Two,   suit: Suit::Spades }),
    //     Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    // );

    // println!("{:?}\n{:?}", hand, hand.strength());

    simulate(1_000_000);
}

fn simulate(iterations: u32) {
    println!("Simulating {iterations} hands.");

    let keys = vec![
        Ranking::RoyalFlush,
        Ranking::StraightFlush,
        Ranking::FourOfAKind,
        Ranking::Flush,
        Ranking::Straight,
        Ranking::ThreeOfAKind,
        Ranking::TwoPair,
        Ranking::OnePair,
        Ranking::HighCard,
    ];
    let mut results: HashMap<Ranking, usize> = keys.into_iter().map(|key| (key, 0)).collect();

    let start_time = Instant::now();

    for i in 0..iterations {
        let mut deck = Deck::new();
        
        deck.shuffle();

        let hand = Hand::new(
            (deck.pop().unwrap(), deck.pop().unwrap()),
            Some((deck.pop().unwrap(), deck.pop().unwrap(), deck.pop().unwrap())),
            deck.pop(),
            deck.pop());

        let strength = hand.strength();

        let result = results.entry(strength.ranking).or_insert(0);
        *result += 1;

        if i % 10_000 == 0 {
            print!(".");
            io::stdout().flush().unwrap_or_default();
        }
    }

    println!();

    let mut sorted_results: Vec<_> = results.into_iter().collect();
    sorted_results.sort_by_key(|entry| entry.0);
    sorted_results.reverse();

    for (ranking, count) in sorted_results {
        let probability = ranking.probability();
        let impiric_probability = count as f32 / iterations as f32;
        println!("{:>15}: {:>10.6} %  {:>10.6} %  (Δ {:>+10.6} %)", ranking.name(), impiric_probability * 100., probability * 100., (impiric_probability - probability) * 100.);
    }

    let elapsed_time = start_time.elapsed();

    println!();
    println!("Simulation time: {:9.3} seconds ({:.3} us per iteration)",
        elapsed_time.as_secs_f32(),
        (elapsed_time.as_secs_f32() / iterations as f32) * 1_000_000f32);
}
