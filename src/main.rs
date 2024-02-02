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
        println!("{} ({})\n p = {:.6} %", ranking, ranking.combinations(), ranking.probability() * 100f32);
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
    let hand = Hand::new(
        (Card { face: Face::Ten,   suit: Suit::Spades }, Card { face: Face::Nine,   suit: Suit::Spades }),
        Some((Card { face: Face::Queen,   suit: Suit::Spades }, Card { face: Face::King,  suit: Suit::Spades }, Card { face: Face::Jack,  suit: Suit::Spades })),
        Some(Card { face: Face::Two,   suit: Suit::Spades }),
        Some(Card { face: Face::Seven,   suit: Suit::Diamonds }),
    );

    println!("{:?}\n{:?}", hand, hand.strength());
}
