use std::{fmt::Display, io};

use super::{
    card::Card,
    score::{compute_oudlers, compute_points},
    utils::compare,
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Bid {
    Petite,
    Garde,
    GardeSans,
    GardeContre,
    #[default]
    Passe,
}
impl Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bid::Petite => write!(f, "1. Petite"),
            Bid::Garde => write!(f, "2. Garde"),
            Bid::GardeSans => write!(f, "3. Garde Sans"),
            Bid::GardeContre => write!(f, "4. Garde Contre"),
            Bid::Passe => write!(f, "5. Passe"),
        }
    }
}

pub fn bot_bid(cards: &[Card], previous_bid: &Bid) -> Bid {
    let n_oudlers = compute_oudlers(cards) as f64;
    let hand_score = compute_points(cards) % 5.0;
    let evaluation = n_oudlers * hand_score;
    let bid = match evaluation {
        0.0..2.0 => Bid::Passe,
        2.0..4.0 => Bid::Petite,
        4.0..6.0 => Bid::Garde,
        6.0..8.0 => Bid::GardeSans,
        _ => Bid::GardeContre,
    };
    match compare(&bid, Some(previous_bid), compare_bids) {
        true => bid,
        false => Bid::Passe,
    }
}

fn prompt_bid(current_bid: &Bid) -> Option<Bid> {
    println!("What is your bid?");
    display_available_bids(&get_available_bids(current_bid));
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read line failed.");
    let name = name.trim();
    match name {
        "1" | "petite" | "Petite" | "PETITE" => Some(Bid::Petite),
        "2" | "garde" | "Garde" | "GARDE" => Some(Bid::Garde),
        "3" | "garde-sans" | "GardeSans" | "GARDE-SANS" => Some(Bid::GardeSans),
        "4" | "garde-contre" | "GardeContre" | "GARDE-CONTRE" => Some(Bid::GardeContre),
        "5" | "passe" | "Passe" | "PASSE" => Some(Bid::Passe),
        _ => None,
    }
}

pub fn human_bid(cards: &[Card], previous_bid: &Bid) -> Bid {
    let bid = prompt_bid(previous_bid);
    match bid {
        Some(bid) => {
            if bid == Bid::Passe || compare(&bid, Some(previous_bid), compare_bids) {
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

pub fn compare_bids(bid: &Bid, active_bid: &Bid) -> bool {
    match bid {
        Bid::Passe => bid == active_bid,
        Bid::Petite => [Bid::Passe].contains(&active_bid),
        Bid::Garde => [Bid::Passe, Bid::Petite].contains(&active_bid),
        Bid::GardeSans => [Bid::Passe, Bid::Petite, Bid::Garde].contains(&active_bid),
        Bid::GardeContre => {
            [Bid::Passe, Bid::Petite, Bid::Garde, Bid::GardeSans].contains(&active_bid)
        }
    }
}

fn get_available_bids(active_bid: &Bid) -> Vec<Bid> {
    let bids = vec![Bid::Petite, Bid::Garde, Bid::GardeSans, Bid::GardeContre];
    let mut available_bids: Vec<Bid> = bids
        .into_iter()
        .filter(|bid| compare(bid, Some(active_bid), compare_bids))
        .collect();
    available_bids.push(Bid::Passe);

    available_bids
}

fn display_available_bids(available_bids: &[Bid]) {
    for bid in available_bids {
        println!("{bid}");
    }
}
