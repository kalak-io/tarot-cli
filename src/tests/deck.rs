#[cfg(test)]
use super::*;

#[test]
fn deck_has_78_cards() {
    let deck = Deck::new();
    assert_eq!(deck.cards.len(), 78);
}
