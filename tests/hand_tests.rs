#[cfg(test)]
mod hand_tests {
    use rstest::rstest;
    use tarot_cli::common::{
        card::{Card, CardSuits},
        hand::{allowed_poignees, Hand, HandActions, Poignee},
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

    // Thresholds from "La Poignée" and the 3- and 5-player sections of the official rules
    #[rstest]
    fn bot_declares_highest_poignee_allowed_by_trump_count(
        #[values(
            (4, 9, None),
            (4, 10, Some(Poignee::Simple)),
            (4, 12, Some(Poignee::Simple)),
            (4, 13, Some(Poignee::Double)),
            (4, 15, Some(Poignee::Triple)),
            (3, 12, None),
            (3, 13, Some(Poignee::Simple)),
            (3, 18, Some(Poignee::Triple)),
            (5, 7, None),
            (5, 8, Some(Poignee::Simple)),
            (5, 13, Some(Poignee::Triple)),
        )]
        case: (usize, u8, Option<Poignee>),
    ) {
        let (n_players, n_trumps, expected) = case;
        let mut hand = make_hand(
            (1..=n_trumps)
                .map(|rank| Card::new(rank, CardSuits::Trumps))
                .collect(),
        );
        hand.bot_declare_poignee(n_players);
        assert_eq!(hand.bonus_poignee, expected);
    }

    #[test]
    fn allowed_poignees_lists_every_poignee_up_to_the_highest() {
        assert_eq!(
            allowed_poignees(13, 4),
            vec![Poignee::Simple, Poignee::Double]
        );
        assert!(allowed_poignees(9, 4).is_empty());
    }
}
