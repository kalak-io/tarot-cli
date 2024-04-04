use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{self, Rng};

use crate::tarot::game::Player;

const SUITS: [(&str, char); 4] = [("Spades", '♠'), ("Hearts", '♥'), ("Diamonds", '♦'), ("Clubs", '♣')];
const TRUMPS: [(&str, char); 1] = [("Trumps", '*')];

enum Kind {
    Suits,
    Trumps,
}

const MIN_NUMBER_CARDS_SPLIT: u8 = 3;
const MAX_NUMBER_CARDS_SPLIT: u8 = 78 - MIN_NUMBER_CARDS_SPLIT;

#[derive(Debug, Clone)]
struct Suit {
    name: String,
    icon: char,
}

#[derive(Debug, Clone)]
pub struct Card {
    rank: u8,
    suit: Suit,
}
impl Card {
    fn is_trump(&self) -> bool {
        self.suit.name == "Trumps"
    }
    fn score(&self) -> f64 {
        if self.is_trump() {
            match self.rank {
                1 | 21 | 22 => 4.5,
                _ => 0.5
            }
        } else {
            match self.rank {
              1..=10 => 0.5,
              11 => 1.5,
              12 => 2.5,
              13 => 3.5,
              14 => 4.5,
              _ => 0.0
            }
        }
    }
    fn name(&self) -> String {
        if self.is_trump() {
            match self.rank {
                1 => String::from("Petit"),
                22 => String::from("Joker"),
                _ => String::from(self.rank.to_string()),
            }
        } else {
            match self.rank {
                11 => String::from("Jack"),
                12 => String::from("Knight"),
                13 => String::from("Queen"),
                14 => String::from("King"),
                _ => String::from(self.rank.to_string()),
            }
        }
    }
    fn id(&self) -> String {
        format!("|{}{}|", self.suit.icon, self.name())
    }
}

fn generate_cards(n_cards: u8, source: Vec<(&str, char)>) -> Vec<Card> {
    let mut cards = vec![];
    for (name, icon) in source {
        for rank in 1..=n_cards {
            // TODO: one instance could be share with
            let suit = Suit { name: name.to_string(), icon };
            let card = Card { rank, suit };
            cards.push(card);
        }
    }
    cards
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub dealer: Option<u8>,
}
impl Deck {
    pub fn new() -> Self {
        let suit_cards = generate_cards(14, SUITS.to_vec());
        let trump_cards = generate_cards(22, TRUMPS.to_vec());
        let mut cards = [suit_cards, trump_cards].concat();
        cards.shuffle(&mut thread_rng());

        Deck {
            cards,
            dealer: None,
        }
    }
    pub fn update_dealer(&mut self, n_players: u8) {
        self.dealer = match self.dealer {
            None => { Some(rand::thread_rng().gen_range(0..n_players) as u8) },
            Some(dealer) => { Some((dealer + 1) % n_players as u8) },
        }
    }
    pub fn split(&mut self) {
        // The pack of cards must be cut in two, taking or leaving must have more than 3 cards.
        let split_index = rand::thread_rng().gen_range(1..MAX_NUMBER_CARDS_SPLIT) as usize;
        let first_slice = &self.cards[..split_index];
        let second_slice = &self.cards[split_index..];
        self.cards = [second_slice, first_slice].concat();
    }
    pub fn deal(&self, players: Vec<Player>, kitty: Vec<Card>) {
        println!("{:?}", players);
        println!("{:?}", kitty);
    }
}

#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/deck.rs"]
mod desk;
