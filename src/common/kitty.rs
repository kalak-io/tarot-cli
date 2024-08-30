use crate::common::utils::display;

use super::{
    card::Card,
    utils::{prompt_selection, subtract},
};

#[derive(Debug, Default)]
pub struct Kitty {
    pub cards: Vec<Card>,
    pub max_size: usize,
}
impl Kitty {
    pub fn new(n_players: usize) -> Self {
        Kitty {
            max_size: get_max_size_kitty(n_players),
            ..Default::default()
        }
    }

    pub fn bot_compose(&mut self, cards: &[Card]) -> Vec<Card> {
        println!("Bot compose kitty");
        cards.to_vec()
    }

    pub fn human_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
        let mut new_kitty: Vec<Card> = Vec::new();
        while new_kitty.len() < self.max_size {
            // get diff of vects cards and new_kitty
            subtract(cards, &new_kitty);

            let index = prompt_selection("Compose your kitty", Some(cards.to_vec()));
            // TODO: implement rule to reject some cards in kitty like Kings0 or Trumps
            new_kitty.push(cards[index].clone());
        }
        println!("The new kitty is:");
        display(&new_kitty);
        self.cards = new_kitty;
        self.cards.clone()
    }
}

pub fn get_max_size_kitty(n_players: usize) -> usize {
    match n_players {
        2..=4 => 6,
        5.. => 3,
        _ => 0,
    }
}
