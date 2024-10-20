use std::fmt::Display;

use crate::common::{card::CardSuits, utils::select};

use super::{
    bid::{Bid, Bids},
    card::{Card, KING_RANK},
    hand::Hand,
    kitty::{Kitty, KittyActions},
    trick::{Trick, TrickActions},
    utils::display,
};

pub trait PlayerActions {
    fn bid(&self, bid: &mut Bid) -> Bids;
    fn call_king(&mut self) -> Card;
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card>;
    fn play(&mut self, trick: &mut Trick);
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub name: String,
    score: u8,
    pub is_human: bool,
    pub is_dealer: bool,
    pub cards: Vec<Card>,
    pub picked_up_cards: Vec<Card>,
    pub hand: Hand,
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
}
impl PlayerActions for Player {
    fn bid(&self, bid: &mut Bid) -> Bids {
        if self.is_human {
            bid.human_choose(&self.hand.cards)
        } else {
            bid.bot_choose(&self.hand.cards)
        }
    }
    fn call_king(&mut self) -> Card {
        let kings: [Card; 4] = [
            Card::new(KING_RANK, CardSuits::Clubs),
            Card::new(KING_RANK, CardSuits::Diamonds),
            Card::new(KING_RANK, CardSuits::Hearts),
            Card::new(KING_RANK, CardSuits::Spades),
        ];
        if self.is_human {
            human_call_king(&self.hand.cards, &kings)
        } else {
            bot_call_king(&self.hand.cards, &kings)
        }
    }
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card> {
        add_kitty_in_hand(&kitty.cards, &mut self.hand);
        if self.is_human {
            kitty.human_compose(&mut self.hand.cards)
        } else {
            kitty.bot_compose(&self.hand.cards)
        }
    }
    fn play(&mut self, trick: &mut Trick) {
        if self.is_human {
            trick.human_play(&mut self.hand.cards)
        } else {
            trick.bot_play(&mut self.hand.cards)
        }
    }
}

fn bot_call_king(_cards: &[Card], _kings: &[Card]) -> Card {
    todo!()
}

fn human_call_king(cards: &[Card], kings: &[Card]) -> Card {
    println!("\nYour cards:");
    display(cards);
    select(Some("Which king do you call?"), Some(kings.to_vec())).unwrap()
}

fn add_kitty_in_hand(kitty: &[Card], hand: &mut Hand) {
    let mut cards = hand.cards.to_vec();
    cards.extend_from_slice(kitty);
    cards.sort_unstable_by_key(|card| (card.suit.initial, card.rank));
    hand.cards = cards
}
