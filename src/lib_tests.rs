#[cfg(test)]
mod deck {
    use super::super::*;

    #[test]
    fn created_deck_has_78_cards() {
        let deck = build_deck();
        assert_eq!(deck.len(), 78);
    }

    #[test]
    fn deck_contains_22_trump_cards() {
        let deck = build_deck();
        let trump_cards = deck
            .iter()
            .filter(|c| c.suit.name == "Trumps")
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(trump_cards, 22);
    }

    #[test]
    fn deck_contains_5_different_suits() {
        // Four suits + Trumps
        let mut deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        assert_eq!(suits.len(), 5);
    }

    #[test]
    fn deck_contains_14_cards_by_suit() {
        let deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        suits.retain(|suit| *suit != "Trumps");
        for suit in suits {
            let suit_cards = deck
                .iter()
                .filter(|c| c.suit.name == suit)
                .collect::<Vec<&Card>>();
            assert_eq!(deck.iter().filter(|c| c.suit.name == suit).count(), 14);
        }
    }

    #[test]
    fn deck_score_equals_91() {
        let deck = build_deck();
        let pdeck = deck.iter().collect::<Vec<&Card>>();
        assert_eq!(compute_score(&pdeck), 91.0);
    }

    #[test]
    fn trumps_score_equals_23() {
        let deck = build_deck();
        let trump_cards = deck
            .iter()
            .filter(|c| c.suit.name == "Trumps")
            .collect::<Vec<&Card>>();
        assert_eq!(compute_score(&trump_cards), 23.0);
    }

    #[test]
    fn suit_score_equals_17() {
        let deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        suits.retain(|suit| *suit != "Trumps");
        for suit in suits {
            let suit_cards = deck
                .iter()
                .filter(|c| c.suit.name == suit)
                .collect::<Vec<&Card>>();
            assert_eq!(compute_score(&suit_cards), 17.0);
        }
    }
}

#[cfg(test)]
mod kitty {
    use super::super::*;

    #[test]
    fn kitty_size_computes_correctly() {
        assert_eq!(kitty_size(1), 0);
        assert_eq!(kitty_size(2), 6);
        assert_eq!(kitty_size(3), 6);
        assert_eq!(kitty_size(4), 6);
        assert_eq!(kitty_size(5), 3);
        assert_eq!(kitty_size(6), 3);
        assert_eq!(kitty_size(7), 3);
    }
}
