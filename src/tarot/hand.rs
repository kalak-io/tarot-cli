use crate::tarot::deck::{Card, Deck};
use crate::tarot::game::Player;

#[derive(Debug)]
pub struct Hand {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub kitty: Vec<Card>,
}
impl Hand {
    pub fn new(players: Vec<Player>, deck: Deck, kitty: Vec<Card>) -> Self {
        Hand {
            players,
            deck,
            kitty,
        }
    }
}
