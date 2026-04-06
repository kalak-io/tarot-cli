use crate::common::{
    card::{CardGetters, CardSuits, KING_RANK},
    utils::display_cards,
};

use super::{
    card::Card,
    utils::{select_card, subtract},
};

pub trait KittyActions {
    fn bot_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card>;
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
    fn bot_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
        // Collect cards eligible for the kitty: no oudlers, no non-trump kings
        let mut candidates: Vec<Card> = cards
            .iter()
            .filter(|card| {
                !(card.is_oudler() || card.rank == KING_RANK && card.suit.name != CardSuits::Trumps)
            })
            .cloned()
            .collect();

        // Prefer non-trumps first, then sort by score ascending (cheapest first)
        candidates.sort_by(|a, b| {
            let a_trump = (a.suit.name == CardSuits::Trumps) as u8;
            let b_trump = (b.suit.name == CardSuits::Trumps) as u8;
            a_trump
                .cmp(&b_trump)
                .then_with(|| a.score().partial_cmp(&b.score()).unwrap())
        });

        let new_kitty: Vec<Card> = candidates.into_iter().take(self.max_size).collect();
        subtract(cards, &new_kitty);
        self.cards = new_kitty.clone();
        new_kitty
    }

    fn human_compose(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
        loop {
            let mut new_kitty: Vec<Card> = Vec::new();
            while new_kitty.len() < self.max_size {
                println!("\nThe building kitty contains: ");
                display_cards(&new_kitty);
                let mut available = cards.clone();
                subtract(&mut available, &new_kitty);

                let card = select_card(Some("Compose your kitty"), Some(available)).unwrap();
                if (card.suit.name != CardSuits::Trumps && card.rank == KING_RANK)
                    || card.is_oudler()
                {
                    println!("You cannot select a King or an Oudler for the kitty.");
                } else {
                    new_kitty.push(card);
                }
            }
            println!("\nThe new kitty is:");
            display_cards(&new_kitty);
            println!("Are you satisfied with this kitty? (yes/no)");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => {
                    subtract(cards, &new_kitty);
                    self.cards = new_kitty;
                    return self.cards.clone();
                }
                _ => println!("Let's redo the kitty selection."),
            }
        }
    }
}
