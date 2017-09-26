use data::{Card, Trick};
use player::Player;

pub struct Game {
    players: [Box<Player>; 4],
    state: GameState,
}

/// A struct that encapsulates all the state of a game of Hearts
pub struct GameState {
    pub past_tricks: Vec<Trick>,
    pub current_trick: Vec<Card>,
}
impl GameState {
    fn new() -> GameState {
        GameState {
            past_tricks: Vec::new(),
            current_trick: Vec::new(),
        }
    }
}
