use super::{
    bid::Bids,
    card::{Card, CardGetters},
    chelem::{Chelem, ChelemResult},
    hand::{Poignee, Side},
};

pub const BASE_SCORE: f64 = 25.0;

pub fn compute_oudlers(cards: &[Card]) -> usize {
    cards.iter().filter(|card| card.is_oudler()).count()
}

pub fn compute_points(cards: &[Card]) -> f64 {
    cards.iter().map(|card| card.score()).sum()
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
        Bids::Take => 1.0,
        Bids::Guard => 2.0,
        Bids::GuardWithout => 4.0,
        Bids::GuardAgainst => 6.0,
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
        None => 0.0,
    }
}

pub fn points_chelem(bonus_chelem: Option<Chelem>) -> f64 {
    match bonus_chelem {
        Some(chelem) => match chelem.result {
            Some(ChelemResult::AnnouncedAndSucceed) => 400.0,
            Some(ChelemResult::NotAnnouncedAndSucceed) => 200.0,
            Some(ChelemResult::AnnouncedAndLost) => -200.0,
            None => 0.0,
        },
        None => 0.0,
    }
}

pub fn compute_score(
    cards: &[Card],
    bid: &Bids,
    bonus_petit_au_bout: Option<Side>,
    bonus_chelem: Option<Chelem>,
    bonus_poignee: Option<Poignee>,
) -> f64 {
    let points = diff_points(cards);
    let multiplier = multiplier(bid);
    let points_petit_au_bout = points_petit_au_bout(bonus_petit_au_bout);
    let points_poignee = points_poignee(bonus_poignee);
    let points_chelem = points_chelem(bonus_chelem);
    (BASE_SCORE + points + points_petit_au_bout) * multiplier + points_poignee + points_chelem
}
