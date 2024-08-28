use super::{bid::Bids, card::Card};

pub fn compute_oudlers(cards: &[Card]) -> usize {
    cards.into_iter().filter(|card| card.is_oudler).count()
}

pub fn compute_points(cards: &[Card]) -> f64 {
    cards.into_iter().fold(0.0, |acc, card| acc + card.score)
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

// fn compute_score(hand: &Hand) -> f64 {
//     let points = diff_points(&hand.attack_pool);
//     let petit_au_bout = if hand.bonus_petit_au_bout { 10.0 } else { 0.0 };
//     ((25.0 + points + petit_au_bout) * multiplier(hand.bid))
//         + hand.bonus_poignee
//         + hand.bonus_chelem
// }
