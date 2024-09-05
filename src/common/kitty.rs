use crate::common::{
    card::{CardGetters, CardSuits, KING_RANK},
    utils::display,
};

use super::{
    card::Card,
    utils::{select, subtract},
};

pub trait KittyActions {
    fn bot_compose(&mut self, cards: &[Card]) -> Vec<Card>;
    fn human_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card>;
}

#[derive(Debug, Default)]
pub struct Kitty {
    pub cards: Vec<Card>,
    pub max_size: usize,
}
impl Kitty {
    pub fn new(n_players: usize) -> Self {
        Kitty {
            max_size: match n_players {
                2..=4 => 6,
                5.. => 3,
                _ => 0,
            },
            ..Default::default()
        }
    }
}

impl KittyActions for Kitty {
    fn bot_compose(&mut self, cards: &[Card]) -> Vec<Card> {
        println!("Bot compose kitty");
        cards.to_vec()
    }

    fn human_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
        let mut new_kitty: Vec<Card> = Vec::new();
        while new_kitty.len() < self.max_size {
            // get diff of vects cards and new_kitty
            subtract(cards, &new_kitty);
            let available_cards = cards.clone();

            let card = select(Some("Compose your kitty"), Some(available_cards)).unwrap();
            if (card.suit.name != CardSuits::Trumps && card.rank == KING_RANK) || card.is_oudler() {
                println!("You cannot select a King or an Oudler for the kitty.");
                continue;
            } else {
                new_kitty.push(card);
            }
            println!("The building kitty contains: ");
            display(&new_kitty);
        }
        println!("The new kitty is:");
        display(&new_kitty);
        self.cards = new_kitty;
        self.cards.clone()
    }
}
