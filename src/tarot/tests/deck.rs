#[cfg(test)]
use super::*;

#[test]
fn deck_has_78_cards() {
    let deck = Deck::new();
    assert_eq!(deck.cards.len(), 78);
}

#[test]
fn deck_contains_22_trump_cards() {
    let deck = Deck::new();
    let trump_cards = deck.cards.iter()
        .filter(|card| card.is_trump())
        .collect::<Vec<_>>()
        .len();
    assert_eq!(trump_cards, 22)
}

#[test]
fn deck_contains_56_suit_cards() {
    let deck = Deck::new();
    let suit_cards = deck.cards.iter()
        .filter(|card| !card.is_trump())
        .collect::<Vec<_>>()
        .len();
    assert_eq!(suit_cards, 56)
}

#[test]
fn deck_contains_14_cards_by_suit() {
    let deck = Deck::new();
    for suit in SUITS {
        let suit_cards = deck.cards.iter()
            .filter(|card| !card.is_trump() && card.suit.name == suit.0)
            .collect::<Vec<_>>()
            .len();
        assert_eq!(suit_cards, 14)
    }
}

#[test]
fn update_dealer() {
    let mut deck = Deck::new();
    deck.update_dealer(5);
    assert!(Some(0) <= deck.dealer && deck.dealer < Some(5));
    deck.update_dealer(5);
    assert!(Some(0) <= deck.dealer && deck.dealer < Some(5));
}
