use super::{
    bid::Bids,
    card::{Card, CardGetters},
    hand::{Chelem, Poignee, Side},
};

pub const BASE_SCORE: f64 = 25.0;

pub fn compute_oudlers(cards: &[Card]) -> usize {
    cards.iter().filter(|card| card.is_oudler()).count()
}

pub fn compute_points(cards: &[Card]) -> f64 {
    cards.iter().fold(0.0, |acc, card| acc + card.score())
}

fn compute_needed_points(cards: &[Card]) -> f64 {
    get_needed_points(compute_oudlers(cards))
}

fn diff_points(cards: &[Card]) -> f64 {
    let points = compute_points(cards);
    let needed_points = compute_needed_points(cards);
    points - needed_points
}

fn get_needed_points(n_oudlers: usize) -> f64 {
    match n_oudlers {
        0 => 56.0,
        1 => 51.0,
        2 => 41.0,
        3 => 36.0,
        _ => 0.0, // maybe raise an error
    }
}

fn multiplier(bid: &Bids) -> f64 {
    match bid {
        Bids::Petite => 1.0,
        Bids::Garde => 2.0,
        Bids::GardeSans => 4.0,
        Bids::GardeContre => 6.0,
        _ => 0.0,
    }
}

pub fn points_petit_au_bout(bonus_petit_au_bout: Option<Side>) -> f64 {
    match bonus_petit_au_bout {
        Some(Side::Attack) => 10.0,
        _ => 0.0,
    }
}

pub fn points_poignee(bonus_poignee: Option<Poignee>) -> f64 {
    match bonus_poignee {
        Some(Poignee::Simple) => 20.0,
        Some(Poignee::Double) => 30.0,
        Some(Poignee::Triple) => 40.0,
        _ => 0.0,
    }
}

pub fn points_chelem(bonus_chelem: Option<Chelem>) -> f64 {
    match bonus_chelem {
        Some(Chelem::AnnouncedAndSucceed) => 400.0,
        Some(Chelem::NotAnnouncedAndSucceed) => 200.0,
        Some(Chelem::AnnouncedAndLost) => -200.0,
        _ => 0.0,
    }
}

pub fn compute_score(
    cards: &[Card],
    bid: &Bids,
    bonus_petit_au_bout: Option<Side>,
    bonus_chelem: Option<Chelem>,
    bonus_poignee: Option<Poignee>,
) -> f64 {
    let points = diff_points(&cards);
    let points_petit_au_bout = points_petit_au_bout(bonus_petit_au_bout);
    (BASE_SCORE + points + points_petit_au_bout) * multiplier(bid)
        + points_poignee(bonus_poignee)
        + points_chelem(bonus_chelem)
}
