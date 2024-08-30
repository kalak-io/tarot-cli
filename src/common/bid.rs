use std::fmt::{Display, Formatter, Result};

use super::{
    card::Card,
    score::{compute_oudlers, compute_points},
    utils::{compare, display, prompt_selection},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Bids {
    Petite,
    Garde,
    GardeSans,
    GardeContre,
    #[default]
    Passe,
}
impl Display for Bids {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
impl Bids {
    const BASICS: [Self; 4] = [
        Self::Petite,
        Self::Garde,
        Self::GardeSans,
        Self::GardeContre,
    ];
}

#[derive(Debug, Default)]
pub struct Bid {
    pub current: Bids,
}
impl Display for Bid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.current)
    }
}
impl Bid {
    pub fn new(current: Bids) -> Self {
        Bid { current }
    }
    pub fn get_available_bids(&self) -> Vec<Bids> {
        let mut available_bids: Vec<Bids> = Bids::BASICS
            .into_iter()
            .filter(|bid| compare(bid, Some(&self.current), compare_bids))
            .collect();
        available_bids.push(Bids::Passe);

        available_bids
    }
    pub fn human_choose(&mut self, cards: &[Card]) -> Bids {
        println!("\nYour cards:");
        display(cards);
        let available_bids = self.get_available_bids();
        let index = prompt_selection("What is your bid?", Some(available_bids));
        self.current = self.get_available_bids().get(index).copied().unwrap();
        self.current
    }
    pub fn bot_choose(&mut self, cards: &[Card]) -> Bids {
        let ideal_bid = taker_evaluation(cards);
        match ideal_bid {
            Bids::Passe => ideal_bid,
            _ => {
                let available_bids = self.get_available_bids();
                if available_bids.contains(&ideal_bid) {
                    self.current = ideal_bid;
                    ideal_bid
                } else {
                    Bids::Passe
                }
            }
        }
    }
}

pub fn compare_bids(bid: &Bids, active_bid: &Bids) -> bool {
    match bid {
        Bids::Passe => bid == active_bid,
        Bids::Petite => [Bids::Passe].contains(&active_bid),
        Bids::Garde => [Bids::Passe, Bids::Petite].contains(&active_bid),
        Bids::GardeSans => [Bids::Passe, Bids::Petite, Bids::Garde].contains(&active_bid),
        Bids::GardeContre => {
            [Bids::Passe, Bids::Petite, Bids::Garde, Bids::GardeSans].contains(&active_bid)
        }
    }
}

pub fn taker_evaluation(cards: &[Card]) -> Bids {
    let n_oudlers = compute_oudlers(cards) as f64;
    let hand_score = compute_points(cards) % 5.0;
    let evaluation = n_oudlers * hand_score;
    match evaluation {
        0.0..2.0 => Bids::Passe,
        2.0..4.0 => Bids::Petite,
        4.0..6.0 => Bids::Garde,
        6.0..8.0 => Bids::GardeSans,
        _ => Bids::GardeContre,
    }
}
