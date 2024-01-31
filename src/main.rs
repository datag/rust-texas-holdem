use crate::card::{Card, Face, Suit};

pub mod card;

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
    ];

    for card in cards {
        println!("Your card is {card} ({card:?}) [ {} ]", card.symbol());
    }
}
