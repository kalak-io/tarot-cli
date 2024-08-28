use std::{cmp::Ordering, fmt::Display};

use super::{
    bid::{bot_bid, human_bid, Bid},
    card::Card,
    hand::Hand,
    taker::Taker,
    trick::Trick,
    utils::display,
};

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
    pub fn bid(&self, current_taker: &Taker) -> Bid {
        match self.is_human {
            true => {
                display(&self.hand.cards);
                human_bid(&self.hand.cards, &current_taker.bid)
            }
            false => bot_bid(&self.hand.cards, &current_taker.bid),
        }
    }
    pub fn compose_kitty(&mut self, kitty: &[Card]) -> Vec<Card> {
        let mut cards = self.hand.cards.to_vec();
        cards.extend_from_slice(kitty);
        cards.sort_by(compare_cards);
        self.hand.cards = cards;
        match self.is_human {
            true => human_compose_kitty(&self.hand.cards),
            false => bot_compose_kitty(&self.hand.cards),
        }
    }
    pub fn play(&self, trick: &Trick) {
        match self.is_human {
            true => {}
            false => {}
        }
    }
}

fn bot_compose_kitty(cards: &[Card]) -> Vec<Card> {
    println!("Bot compose kitty");
    cards.to_vec()
}

fn human_compose_kitty(cards: &[Card]) -> Vec<Card> {
    println!("\n\nCompose your kitty");

    display(&cards);
    // select_cards(&cards, get_kitty_expected_size(n_players));
    cards.to_vec()
}

fn compare_cards(a: &Card, b: &Card) -> Ordering {
    // TODO: sort by suit and by rank
    a.id.cmp(&b.id)
}
