extern crate rand;
use rand::{XorShiftRng, SeedableRng};

extern crate hearts;
use hearts::data::Deck;
use hearts::player::{TrivialPlayer, Player};

fn main() {
    let mut prng = XorShiftRng::from_seed([42, 42, 42, 42]);
    let mut deck = Deck::shuffled(prng);
    println!("{:?}", deck);
    let mut hands = deck.deal();
    for hand in hands.iter() {
        println!("{:?}", hand);
    }

    let mut players = [
        Box::new(TrivialPlayer::new()),
        Box::new(TrivialPlayer::new()),
        Box::new(TrivialPlayer::new()),
        Box::new(TrivialPlayer::new()),
    ];

    for (i, hand) in hands.iter().enumerate() {
        players[i].start(hand.clone());
    }
}
