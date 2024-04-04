use crate::tarot::game::Player;
use crate::tarot::deck::Card;

#[derive(Debug)]
pub struct Hand {
    pub players: Vec<Player>,
    pub kitty: Vec<Card>,
}
impl Hand {
    pub fn new(players: Vec<Player>) -> Self {
        Hand {
            players,
            kitty: vec![],
        }
    }
}
