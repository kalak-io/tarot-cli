#[cfg(test)]
use super::*;

#[test]
fn returns_good_score_total_91() {
    let deck = Deck::new();
    let score = compute_score(&deck.cards);
    assert_eq!(score, 91.0);
}
