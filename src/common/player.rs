use std::fmt::Display;

use crate::common::{card::CardSuits, utils::select};

use super::{
    bid::{Bid, Bids},
    card::Card,
    hand::Hand,
    kitty::Kitty,
    trick::Trick,
    utils::display,
};

const KING_RANK: u8 = 14;

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
    pub fn bid(&self, bid: &mut Bid) -> Bids {
        if self.is_human {
            bid.human_choose(&self.hand.cards)
        } else {
            bid.bot_choose(&self.hand.cards)
        }
    }
    pub fn call_king(&mut self) -> Card {
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
    pub fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card> {
        add_kitty_in_hand(&kitty.cards, &mut self.hand);
        if self.is_human {
            kitty.human_compose(&mut self.hand.cards)
        } else {
            kitty.bot_compose(&self.hand.cards)
        }
    }
    pub fn play(&self, trick: &Trick) {
        match self.is_human {
            true => {}
            false => {}
        }
    }
}

fn bot_call_king(cards: &[Card], kings: &[Card]) -> Card {
    todo!()
}

fn human_call_king(cards: &[Card], kings: &[Card]) -> Card {
    println!("\nYour cards:");
    display(cards);
    let index: usize = prompt_selection("Which king do you call?", Some(kings.to_vec()));
    kings[index].clone()
}

fn add_kitty_in_hand(kitty: &[Card], hand: &mut Hand) {
    let mut cards = hand.cards.to_vec();
    cards.extend_from_slice(kitty);
    cards.sort_unstable_by_key(|card| (card.suit.initial, card.rank));
    hand.cards = cards
}
