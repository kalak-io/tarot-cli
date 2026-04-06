#[cfg(test)]
mod hand {
    use tarot_cli::common::{
        card::{Card, CardSuits},
        hand::Hand,
    };

    fn make_hand(cards: Vec<Card>) -> Hand {
        Hand {
            cards,
            ..Hand::default()
        }
    }

    #[test]
    fn sort_by_suit_groups_cards_by_suit() {
        let mut hand = make_hand(vec![
            Card::new(5, CardSuits::Trumps),
            Card::new(3, CardSuits::Hearts),
            Card::new(7, CardSuits::Spades),
            Card::new(1, CardSuits::Clubs),
            Card::new(4, CardSuits::Diamonds),
        ]);
        hand.sort_by_suit();
        let suits: Vec<CardSuits> = hand.cards.iter().map(|c| c.suit.name).collect();
        assert_eq!(
            suits,
            vec![
                CardSuits::Spades,
                CardSuits::Hearts,
                CardSuits::Diamonds,
                CardSuits::Clubs,
                CardSuits::Trumps,
            ]
        );
    }

    #[test]
    fn sort_by_suit_orders_ranks_ascending_within_suit() {
        let mut hand = make_hand(vec![
            Card::new(10, CardSuits::Hearts),
            Card::new(2, CardSuits::Hearts),
            Card::new(7, CardSuits::Hearts),
        ]);
        hand.sort_by_suit();
        let ranks: Vec<u8> = hand.cards.iter().map(|c| c.rank).collect();
        assert_eq!(ranks, vec![2, 7, 10]);
    }

    #[test]
    fn sort_by_suit_places_trumps_last() {
        let mut hand = make_hand(vec![
            Card::new(3, CardSuits::Trumps),
            Card::new(14, CardSuits::Spades),
            Card::new(1, CardSuits::Trumps),
        ]);
        hand.sort_by_suit();
        assert_eq!(hand.cards[0].suit.name, CardSuits::Spades);
        assert_eq!(hand.cards[1].suit.name, CardSuits::Trumps);
        assert_eq!(hand.cards[2].suit.name, CardSuits::Trumps);
    }

    #[test]
    fn sort_by_suit_empty_hand_does_not_panic() {
        let mut hand = make_hand(vec![]);
        hand.sort_by_suit();
        assert!(hand.cards.is_empty());
    }
}
