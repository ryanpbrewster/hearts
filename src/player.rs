use data::{Hand, Card};
use game::GameState;
use util;

pub trait Player {
    fn start(&mut self, hand: Hand);
    fn play(&mut self, game_state: &GameState) -> Card;
}

pub struct TrivialPlayer {
    hand: Hand,
}

impl TrivialPlayer {
    pub fn new() -> TrivialPlayer {
        TrivialPlayer { hand: Hand::new() }
    }
}

impl Player for TrivialPlayer {
    fn start(&mut self, hand: Hand) {
        self.hand = hand;
    }

    fn play(&mut self, game_state: &GameState) -> Card {
        let possible_plays = util::legal_plays(&self.hand, &game_state.current_trick);
        match possible_plays.first() {
            None => panic!("Expected to have a legal play with hand {:?}", self.hand),
            Some(&first_legal_play) => {
                self.hand.cards.remove_item(&first_legal_play);
                first_legal_play
            }
        }
    }
}
