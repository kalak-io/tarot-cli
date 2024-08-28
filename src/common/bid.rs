use std::{
    fmt::{Display, Formatter, Result},
    io,
};

use super::{
    card::Card,
    score::{compute_oudlers, compute_points},
    utils::compare,
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

pub fn bot_bid(cards: &[Card], previous_bid: &Bids) -> Bids {
    let n_oudlers = compute_oudlers(cards) as f64;
    let hand_score = compute_points(cards) % 5.0;
    let evaluation = n_oudlers * hand_score;
    let bid = match evaluation {
        0.0..2.0 => Bids::Passe,
        2.0..4.0 => Bids::Petite,
        4.0..6.0 => Bids::Garde,
        6.0..8.0 => Bids::GardeSans,
        _ => Bids::GardeContre,
    };
    match compare(&bid, Some(previous_bid), compare_bids) {
        true => bid,
        false => Bids::Passe,
    }
}

fn prompt_bid(current_bid: &Bids) -> Option<Bids> {
    println!("What is your bid?");
    display_available_bids(&get_available_bids(current_bid));
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read line failed.");
    let name = name.trim();
    match name {
        "1" | "petite" | "Petite" | "PETITE" => Some(Bids::Petite),
        "2" | "garde" | "Garde" | "GARDE" => Some(Bids::Garde),
        "3" | "garde-sans" | "GardeSans" | "GARDE-SANS" => Some(Bids::GardeSans),
        "4" | "garde-contre" | "GardeContre" | "GARDE-CONTRE" => Some(Bids::GardeContre),
        "5" | "passe" | "Passe" | "PASSE" => Some(Bids::Passe),
        _ => None,
    }
}

pub fn human_bid(cards: &[Card], previous_bid: &Bids) -> Bids {
    let bid = prompt_bid(previous_bid);
    match bid {
        Some(bid) => {
            if bid == Bids::Passe || compare(&bid, Some(previous_bid), compare_bids) {
                bid
            } else {
                human_bid(cards, previous_bid)
            }
        }
        None => {
            println!("Type a number between 1 and 5");
            human_bid(cards, previous_bid)
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

fn get_available_bids(active_bid: &Bids) -> Vec<Bids> {
    let bids = vec![
        Bids::Petite,
        Bids::Garde,
        Bids::GardeSans,
        Bids::GardeContre,
    ];
    let mut available_bids: Vec<Bids> = bids
        .into_iter()
        .filter(|bid| compare(bid, Some(active_bid), compare_bids))
        .collect();
    available_bids.push(Bids::Passe);

    available_bids
}

fn display_available_bids(available_bids: &[Bids]) {
    for bid in available_bids {
        println!("{bid}");
    }
}
