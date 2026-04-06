use std::fmt::{Display, Formatter, Result};

use super::{
    card::Card,
    score::{compute_oudlers, compute_points},
    utils::{compare, display_cards, select},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Bids {
    Take,
    Guard,
    GuardWithout,
    GuardAgainst,
    #[default]
    Pass,
}
impl Display for Bids {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
impl Bids {
    const AVAILABLE_BIDS: [Self; 4] = [
        Self::Take,
        Self::Guard,
        Self::GuardWithout,
        Self::GuardAgainst,
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
        let mut available_bids: Vec<Bids> = Bids::AVAILABLE_BIDS
            .into_iter()
            .filter(|bid| compare(bid, Some(&self.current), compare_bids))
            .collect();
        available_bids.push(Bids::Pass);

        available_bids
    }
    pub fn human_choose(&mut self, cards: &[Card]) -> Bids {
        println!("\nYour cards:");
        display_cards(cards);
        let available_bids = self.get_available_bids();
        self.current = select(Some("What is your bid?"), Some(available_bids)).unwrap();
        self.current
    }
    pub fn bot_choose(&mut self, cards: &[Card]) -> Bids {
        let ideal_bid = taker_evaluation(cards);
        match ideal_bid {
            Bids::Pass => ideal_bid,
            _ => {
                let available_bids = self.get_available_bids();
                if available_bids.contains(&ideal_bid) {
                    self.current = ideal_bid;
                    ideal_bid
                } else {
                    Bids::Pass
                }
            }
        }
    }
}

pub fn compare_bids(bid: &Bids, active_bid: &Bids) -> bool {
    match bid {
        Bids::Pass => bid == active_bid,
        Bids::Take => [Bids::Pass].contains(active_bid),
        Bids::Guard => [Bids::Pass, Bids::Take].contains(active_bid),
        Bids::GuardWithout => [Bids::Pass, Bids::Take, Bids::Guard].contains(active_bid),
        Bids::GuardAgainst => {
            [Bids::Pass, Bids::Take, Bids::Guard, Bids::GuardWithout].contains(active_bid)
        }
    }
}

pub fn taker_evaluation(cards: &[Card]) -> Bids {
    let n_oudlers = compute_oudlers(cards) as f64;
    let hand_score = compute_points(cards) % 5.0;
    let evaluation = n_oudlers * hand_score;
    match evaluation {
        0.0..2.0 => Bids::Pass,
        2.0..4.0 => Bids::Take,
        4.0..6.0 => Bids::Guard,
        6.0..8.0 => Bids::GuardWithout,
        _ => Bids::GuardAgainst,
    }
}
