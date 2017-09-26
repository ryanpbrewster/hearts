use std::fmt::{Formatter, Result, Debug};
use rand::Rng;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Debug for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Suit::Clubs => 'C',
                Suit::Diamonds => 'D',
                Suit::Hearts => 'H',
                Suit::Spades => 'S',
            }
        )
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Rank::Two => '2',
                Rank::Three => '3',
                Rank::Four => '4',
                Rank::Five => '5',
                Rank::Six => '6',
                Rank::Seven => '7',
                Rank::Eight => '8',
                Rank::Nine => '9',
                Rank::Ten => 'T',
                Rank::Jack => 'J',
                Rank::Queen => 'Q',
                Rank::King => 'K',
                Rank::Ace => 'A',
            }
        )
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}{:?}", self.rank, self.suit)
    }
}

pub struct Deck {
    cards: [Card; 52],
}
impl Debug for Deck {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.cards.as_ref().fmt(f)
    }
}
impl Deck {
    fn new() -> Deck {
        // TODO(ryanpbrewster): figure out some better way to code this
        // that still allows the compiler to guarantee there are exactly 52 cards
        Deck {
            cards: [
                Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spades,
                },
            ],
        }
    }

    pub fn shuffled<R>(mut prng: R) -> Deck
    where
        R: Rng,
    {
        let mut deck = Deck::new();
        prng.shuffle(&mut deck.cards);
        deck
    }

    pub fn deal(self) -> [Hand; 4] {
        let mut hands = [Hand::new(), Hand::new(), Hand::new(), Hand::new()];
        for i in 0..self.cards.len() {
            hands[i % 4].cards.push(self.cards[i]);
        }
        hands
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }
}
impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.cards.fmt(f)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Trick {
    pub lead_player: usize,
    pub cards: [Card; 4],
}
