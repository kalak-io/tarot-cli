use std::fmt::{Display, Formatter, Result};

use super::{
    card::{Card, CardGetters, CardSuits, JACK_RANK, KING_RANK, KNIGHT_RANK, QUEEN_RANK},
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
        let choice = select(Some("What is your bid?"), Some(available_bids)).unwrap();
        self.record(choice)
    }
    pub fn bot_choose(&mut self, cards: &[Card], n_players: usize) -> Bids {
        let ideal_bid = taker_evaluation(cards, n_players);
        if self.get_available_bids().contains(&ideal_bid) {
            self.record(ideal_bid)
        } else {
            Bids::Pass
        }
    }
    // A pass leaves the highest bid so far in place
    pub fn record(&mut self, bid: Bids) -> Bids {
        if bid != Bids::Pass {
            self.current = bid;
        }
        bid
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

// Hand strength for bot bids. This is a simple point count, not a rule from the official rules.
// The 21 is worth 10, the Fool 8 and the Little 5. Each other trump is worth 2, plus 1 from the 16 up.
// Kings, queens, knights and jacks are worth 6, 3, 2 and 1. Each suit of 5 cards or more adds 5.
pub fn hand_strength(cards: &[Card]) -> u32 {
    let card_strength: u32 = cards
        .iter()
        .map(|card| match (card.suit.name, card.rank) {
            (CardSuits::Trumps, 21) => 10,
            _ if card.is_fool() => 8,
            (CardSuits::Trumps, 1) => 5,
            (CardSuits::Trumps, 16..=20) => 3,
            (CardSuits::Trumps, _) => 2,
            (_, KING_RANK) => 6,
            (_, QUEEN_RANK) => 3,
            (_, KNIGHT_RANK) => 2,
            (_, JACK_RANK) => 1,
            _ => 0,
        })
        .sum();
    let long_suits = [
        CardSuits::Clubs,
        CardSuits::Diamonds,
        CardSuits::Hearts,
        CardSuits::Spades,
    ]
    .into_iter()
    .filter(|suit| cards.iter().filter(|card| card.suit.name == *suit).count() >= 5)
    .count() as u32;
    card_strength + 5 * long_suits
}

// Minimum hand strength for a Take, a Guard, a Guard Without and a Guard Against.
// With 4 players, about 23% of hands reach a Take, 7% a Guard, 0.7% a Guard Without
// and 0.04% a Guard Against. The 3- and 5-player cutoffs give the same shares for
// their 24- and 15-card hands, measured over 100,000 random deals per player count.
// Bots lose most contracts at these cutoffs. `cargo run --release --bin bench -- --calibrate`
// shows the strengths where each bid wins often enough, but with the current bot card play
// those strengths are so rare that most deals end with everyone passing.
fn bid_cutoffs(n_players: usize) -> [u32; 4] {
    match n_players {
        3 => [50, 57, 65, 72],
        5 => [29, 35, 43, 50],
        _ => [36, 42, 50, 57],
    }
}

pub fn taker_evaluation(cards: &[Card], n_players: usize) -> Bids {
    let [take, guard, guard_without, guard_against] = bid_cutoffs(n_players);
    match hand_strength(cards) {
        strength if strength >= guard_against => Bids::GuardAgainst,
        strength if strength >= guard_without => Bids::GuardWithout,
        strength if strength >= guard => Bids::Guard,
        strength if strength >= take => Bids::Take,
        _ => Bids::Pass,
    }
}
