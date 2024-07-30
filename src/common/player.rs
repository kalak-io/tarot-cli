use std::fmt::Display;

use super::{
    bid::{bot_bid, human_bid, Bid},
    card::Card,
    taker::Taker,
    utils::display,
};

// pub trait PlayerActions {
//     fn bid(&self, current_taker: Taker) -> Bid;
//     fn play(&self);
// }

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub name: String,
    score: u8,
    pub is_human: bool,
    pub is_dealer: bool,
    pub cards: Vec<Card>,
    pub picked_up_cards: Vec<Card>,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}
impl Player {
    pub fn new(name: String, id: u8) -> Self {
        Player {
            id,
            name,
            ..Default::default()
        }
    }
    pub fn bid(&self, current_taker: &Taker) -> Bid {
        match self.is_human {
            true => {
                display(&self.cards);
                human_bid(&self.cards, &current_taker.bid)
            }
            false => bot_bid(&self.cards, &current_taker.bid),
        }
    }
    fn play(&self) {
        match self.is_human {
            true => {}
            false => {}
        }
    }
}
