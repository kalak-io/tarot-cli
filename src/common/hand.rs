use std::fmt::{Display, Formatter, Result};

use super::{
    card::{Card, CardSuits},
    chelem::{Chelem, ChelemState},
    utils::{ask_yes_no, select},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Side {
    Attack,
    #[default]
    Defense,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Poignee {
    Simple,
    Double,
    Triple,
}
impl Display for Poignee {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

// Minimum number of trumps for a simple, double and triple poignee
fn poignee_thresholds(n_players: usize) -> [usize; 3] {
    match n_players {
        3 => [13, 15, 18],
        5 => [8, 10, 13],
        _ => [10, 13, 15],
    }
}

// Poignees a hand holding `trump_count` trumps can declare, lowest first
pub fn allowed_poignees(trump_count: usize, n_players: usize) -> Vec<Poignee> {
    let [simple, double, triple] = poignee_thresholds(n_players);
    [
        (simple, Poignee::Simple),
        (double, Poignee::Double),
        (triple, Poignee::Triple),
    ]
    .into_iter()
    .filter(|(min_trumps, _)| trump_count >= *min_trumps)
    .map(|(_, poignee)| poignee)
    .collect()
}

pub trait HandActions {
    fn human_declare_poignee(&mut self, n_players: usize);
    fn human_declare_chelem(&mut self);
    fn bot_declare_poignee(&mut self, n_players: usize);
    fn bot_declare_chelem(&mut self);
}
#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub won_cards: Vec<Card>,
    pub side: Side,
    pub bonus_poignee: Option<Poignee>,
    pub bonus_chelem: Option<Chelem>,
}
impl Hand {
    pub fn set_side_with_called_card(&mut self, called_card: Option<Card>) {
        if let Some(called_card) = called_card {
            if self.cards.contains(&called_card) {
                self.side = Side::Attack;
            }
        }
    }

    pub fn trump_count(&self) -> usize {
        self.cards
            .iter()
            .filter(|c| c.suit.name == CardSuits::Trumps)
            .count()
    }

    pub fn sort_by_suit(&mut self) {
        self.cards
            .sort_by_key(|c| (suit_order(c.suit.name), c.rank));
    }
}

// Display order for hand sorting: Spades, Hearts, Diamonds, Clubs, Trumps.
// This is the user-facing sort order and intentionally differs from CardSuits::AVAILABLE_SUITS.
fn suit_order(suit: CardSuits) -> u8 {
    match suit {
        CardSuits::Spades => 0,
        CardSuits::Hearts => 1,
        CardSuits::Diamonds => 2,
        CardSuits::Clubs => 3,
        CardSuits::Trumps => 4,
    }
}

impl HandActions for Hand {
    fn human_declare_poignee(&mut self, n_players: usize) {
        if self.bonus_poignee.is_some() {
            return;
        }
        let allowed = allowed_poignees(self.trump_count(), n_players);
        if allowed.is_empty() {
            return;
        }
        if !ask_yes_no("Do you want to declare a poignee?") {
            return;
        }
        self.bonus_poignee = select(Some("Choose a poignee"), Some(allowed));
    }
    fn human_declare_chelem(&mut self) {
        if self.bonus_chelem.is_some() {
            return;
        }
        let state = if ask_yes_no("Do you want to announce a chelem?") {
            ChelemState::Announced
        } else {
            ChelemState::NotAnnounced
        };
        self.bonus_chelem = Some(Chelem {
            state,
            result: None,
        });
    }
    fn bot_declare_poignee(&mut self, n_players: usize) {
        if self.bonus_poignee.is_some() {
            return;
        }
        self.bonus_poignee = allowed_poignees(self.trump_count(), n_players).pop();
    }
    fn bot_declare_chelem(&mut self) {
        self.bonus_chelem = Some(Chelem {
            state: ChelemState::NotAnnounced,
            result: None,
        });
    }
}
