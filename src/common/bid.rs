use std::{fmt::Display, io};

use super::{
    card::Card,
    player::Player,
    score::{compute_oudlers, compute_points},
    taker::Taker,
    utils::{compare, display},
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
            Bid::Petite => write!(f, "Petite"),
            Bid::Garde => write!(f, "Garde"),
            Bid::GardeSans => write!(f, "Garde Sans"),
            Bid::GardeContre => write!(f, "Garde Contre"),
            Bid::Passe => write!(f, "Passe"),
        }
    }
}

pub fn collect_bid(players: &[Player]) -> Taker {
    let mut taker = Taker {
        player: players[0].clone(),
        bid: Bid::Passe,
    };
    for player in players {
        let bid = player.bid(&taker);
        if compare(&bid, Some(&taker.bid), compare_bids) {
            taker.player.id = player.id;
            taker.bid = bid;
        }
        println!("{} make the bid: {}", player.name, bid);
    }
    return taker.clone();
}

pub fn bot_bid(cards: &[Card], previous_bid: &Bid) -> Bid {
    let n_oudlers = compute_oudlers(cards) as f64;
    // println!("{} oudlers", n_oudlers);
    let hand_score = compute_points(cards) % 5.0;
    // println!("Hand score: {}", hand_score);
    let evaluation = n_oudlers * hand_score;
    // println!("Hand evaluation: {}", evaluation);
    let bid = match evaluation {
        0.0..=8.0 => Bid::Passe,
        8.0..=15.0 => Bid::Petite,
        15.0..=500.0 => Bid::Garde, // TODO: GardeContre after update computation of evaluation
        _ => Bid::Passe,
    };
    match compare(&bid, Some(previous_bid), compare_bids) {
        true => bid,
        false => Bid::Passe,
    } // in this case, there is no evaluation of a second bid
}

fn prompt_bid(current_bid: &Bid) -> Option<Bid> {
    println!("What is your bid?");
    // println!("1. Petite, 2. Garde, 3. Garde Sans, 4. Garde Contre, 5. Passe");
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
            if compare(&bid, Some(previous_bid), compare_bids) {
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

fn compare_bids(bid: &Bid, active_bid: &Bid) -> bool {
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
    let bids = vec![
        Bid::Petite,
        Bid::Garde,
        Bid::GardeSans,
        Bid::GardeContre,
        Bid::Passe,
    ];
    bids.into_iter()
        .filter(|bid| compare(bid, Some(active_bid), compare_bids))
        .collect()
}

fn display_available_bids(available_bids: &[Bid]) {
    // TODO: add the right number to press
    let mut index = 1;
    for bid in available_bids {
        println!("{index}. {bid}");
        index += 1;
    }
}
