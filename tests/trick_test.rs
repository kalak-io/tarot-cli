#[cfg(test)]
mod trick {
    use rstest::rstest;
    use tarot_cli::common::{
        card::{Card, CardSuits},
        trick::{allowed_cards_to_play, Trick, TrickActions, TrickGetters},
    };

    #[rstest]
    fn trick_get_best_played_card_index(
        #[values(
            (Vec::new(), None),
            (Vec::from([Card::new(2, CardSuits::Clubs), Card::new(14, CardSuits::Clubs)]), Some(1)),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Some(1)),
            (Vec::from([Card::new(8, CardSuits::Clubs), Card::new(10, CardSuits::Clubs), Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Clubs), Card::new(1, CardSuits::Clubs)]), Some(2)),
            (Vec::from([Card::new(8, CardSuits::Trumps), Card::new(10, CardSuits::Trumps), Card::new(14, CardSuits::Trumps), Card::new(2, CardSuits::Trumps), Card::new(1, CardSuits::Trumps)]), Some(2)),
            (Vec::from([Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs)]), Some(0)),
            // The Fool never wins, even against lower trumps or when it leads
            (Vec::from([Card::new(22, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(5, CardSuits::Clubs)]), Some(2)),
            (Vec::from([Card::new(3, CardSuits::Trumps), Card::new(22, CardSuits::Trumps)]), Some(0)),
            (Vec::from([Card::new(22, CardSuits::Trumps)]), None))]
        case: (Vec<Card>, Option<usize>),
    ) {
        let (played_cards, expected_index) = case;
        let mut trick = Trick::default();
        for played_card in played_cards {
            trick.played_cards.push(played_card);
        }
        assert_eq!(
            trick.get_best_played_card_index(trick.played_suit()),
            expected_index
        );
    }

    #[rstest]
    fn selected_card_is_allowed_in_context(
        #[values(
            (Vec::new(), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), false),
            (Vec::new(), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Clubs), true),
            (Vec::from([Card::new(8, CardSuits::Clubs)]), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Clubs), true),
            (Vec::from([Card::new(8, CardSuits::Clubs)]), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(2, CardSuits::Hearts), false),
            (Vec::from([Card::new(8, CardSuits::Trumps)]), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(2, CardSuits::Hearts), false),
            (Vec::from([Card::new(8, CardSuits::Trumps)]), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(2, CardSuits::Trumps), true),
            (Vec::from([Card::new(8, CardSuits::Trumps)]), Vec::from([Card::new(14, CardSuits::Trumps), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Trumps), true),
            (Vec::from([Card::new(8, CardSuits::Trumps)]), Vec::from([Card::new(14, CardSuits::Trumps), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(2, CardSuits::Trumps), false),
            (Vec::from([Card::new(8, CardSuits::Trumps)]), Vec::from([Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(2, CardSuits::Hearts), true),
        )]
        case: (Vec<Card>, Vec<Card>, Card, bool),
    ) {
        let (played_cards, player_cards, selected_card, expected_result) = case;
        let trick = Trick {
            played_cards,
            ..Default::default()
        };
        assert_eq!(
            allowed_cards_to_play(&trick, &player_cards).contains(&selected_card),
            expected_result
        );
    }

    #[rstest]
    fn bot_play_leads_with_cheap_card(
        #[values(
            // Hand has King, Jack and a cheap card: plays the cheap card
            (Vec::new(), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(11, CardSuits::Clubs), Card::new(5, CardSuits::Clubs)]), Card::new(5, CardSuits::Clubs)),
            // Hand has Fool (oudler) and cheap cards: avoids the oudler
            (Vec::new(), Vec::from([Card::new(22, CardSuits::Trumps), Card::new(5, CardSuits::Spades), Card::new(7, CardSuits::Hearts)]), Card::new(5, CardSuits::Spades)),
            // Hand has King and a low trump: prefers cheap non-trump over King
            (Vec::new(), Vec::from([Card::new(14, CardSuits::Clubs), Card::new(5, CardSuits::Spades), Card::new(3, CardSuits::Trumps)]), Card::new(5, CardSuits::Spades)),
        )]
        case: (Vec<Card>, Vec<Card>, Card),
    ) {
        let (played_cards, mut hand, expected_card) = case;
        let mut trick = Trick {
            played_cards,
            ..Default::default()
        };
        trick.bot_play(&mut hand);
        assert_eq!(trick.played_cards.last().unwrap(), &expected_card);
    }

    #[rstest]
    fn bot_play_following_wins_valuable_trick_with_cheapest_card(
        #[values(
            // Jack played (value 1.5), hand has losing + two winners: picks cheapest winner (Queen over King)
            (Vec::from([Card::new(11, CardSuits::Clubs)]), Vec::from([Card::new(5, CardSuits::Clubs), Card::new(13, CardSuits::Clubs), Card::new(14, CardSuits::Clubs)]), Card::new(13, CardSuits::Clubs)),
            // King played (value 4.5), hand must trump: spares Petit, plays low trump instead
            (Vec::from([Card::new(14, CardSuits::Clubs)]), Vec::from([Card::new(1, CardSuits::Trumps), Card::new(15, CardSuits::Trumps)]), Card::new(15, CardSuits::Trumps)),
        )]
        case: (Vec<Card>, Vec<Card>, Card),
    ) {
        let (played_cards, mut hand, expected_card) = case;
        let mut trick = Trick {
            played_cards,
            ..Default::default()
        };
        trick.bot_play(&mut hand);
        assert_eq!(trick.played_cards.last().unwrap(), &expected_card);
    }

    #[rstest]
    fn bot_play_following_discards_cheapest_card(
        #[values(
            // King already played, nothing beats it: discards cheapest club
            (Vec::from([Card::new(14, CardSuits::Clubs)]), Vec::from([Card::new(13, CardSuits::Clubs), Card::new(5, CardSuits::Clubs)]), Card::new(5, CardSuits::Clubs)),
            // Cheap trick (value < 1.5), not worth winning: discards cheapest
            (Vec::from([Card::new(3, CardSuits::Clubs)]), Vec::from([Card::new(13, CardSuits::Clubs), Card::new(11, CardSuits::Clubs)]), Card::new(11, CardSuits::Clubs)),
        )]
        case: (Vec<Card>, Vec<Card>, Card),
    ) {
        let (played_cards, mut hand, expected_card) = case;
        let mut trick = Trick {
            played_cards,
            ..Default::default()
        };
        trick.bot_play(&mut hand);
        assert_eq!(trick.played_cards.last().unwrap(), &expected_card);
    }

    #[test]
    fn allowed_cards_no_trick_yet_all_cards_allowed() {
        let trick = Trick::default();
        let hand = vec![
            Card::new(14, CardSuits::Hearts),
            Card::new(5, CardSuits::Spades),
            Card::new(3, CardSuits::Trumps),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, hand);
    }

    #[test]
    fn allowed_cards_must_follow_suit_when_held() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Clubs)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(2, CardSuits::Clubs),
            Card::new(5, CardSuits::Clubs),
            Card::new(3, CardSuits::Hearts),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(
            allowed,
            vec![
                Card::new(2, CardSuits::Clubs),
                Card::new(5, CardSuits::Clubs)
            ]
        );
    }

    #[test]
    fn allowed_cards_must_trump_when_no_suit() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Clubs)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(5, CardSuits::Hearts),
            Card::new(7, CardSuits::Trumps),
            Card::new(3, CardSuits::Hearts),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, vec![Card::new(7, CardSuits::Trumps)]);
    }

    #[test]
    fn allowed_cards_any_card_when_no_suit_no_trump() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Clubs)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(5, CardSuits::Hearts),
            Card::new(3, CardSuits::Diamonds),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, hand);
    }

    #[test]
    fn allowed_cards_trump_led_must_overtrump_when_possible() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Trumps)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(10, CardSuits::Trumps),
            Card::new(5, CardSuits::Trumps),
            Card::new(2, CardSuits::Trumps),
        ];
        // Only rank-10 beats the rank-8 already played
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, vec![Card::new(10, CardSuits::Trumps)]);
    }

    #[test]
    fn allowed_cards_trump_led_all_trumps_when_cannot_overtrump() {
        let trick = Trick {
            played_cards: vec![Card::new(10, CardSuits::Trumps)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(5, CardSuits::Trumps),
            Card::new(2, CardSuits::Trumps),
        ];
        // No superior trump available → all trumps allowed
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, hand);
    }

    #[test]
    fn allowed_cards_trump_led_no_trumps_any_card() {
        let trick = Trick {
            played_cards: vec![Card::new(10, CardSuits::Trumps)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(5, CardSuits::Hearts),
            Card::new(3, CardSuits::Diamonds),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, hand);
    }

    #[rstest]
    fn has_petit_au_bout(
        #[values(
        (Vec::new(), false),
        (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), false),
        (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(1, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), true),
    )]
        case: (Vec<Card>, bool),
    ) {
        let (played_cards, expected_result) = case;
        let trick = Trick {
            played_cards,
            ..Default::default()
        };
        assert_eq!(trick.has_petit_au_bout(), expected_result);
    }

    #[test]
    fn played_suit_is_set_by_the_card_after_a_leading_fool() {
        let mut trick = Trick::default();
        trick.played_cards.push(Card::new(22, CardSuits::Trumps));
        assert_eq!(trick.played_suit(), None);
        trick.played_cards.push(Card::new(5, CardSuits::Hearts));
        assert_eq!(trick.played_suit(), Some(CardSuits::Hearts));
    }

    #[test]
    fn allowed_cards_fool_allowed_while_holding_led_suit() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Clubs)],
            ..Default::default()
        };
        let hand = vec![
            Card::new(2, CardSuits::Clubs),
            Card::new(22, CardSuits::Trumps),
            Card::new(3, CardSuits::Hearts),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(
            allowed,
            vec![
                Card::new(2, CardSuits::Clubs),
                Card::new(22, CardSuits::Trumps)
            ]
        );
    }

    #[test]
    fn allowed_cards_fool_does_not_count_as_a_trump_to_follow() {
        let trick = Trick {
            played_cards: vec![Card::new(8, CardSuits::Trumps)],
            ..Default::default()
        };
        // No trump other than the Fool: any card can be played
        let hand = vec![
            Card::new(5, CardSuits::Hearts),
            Card::new(22, CardSuits::Trumps),
        ];
        let allowed = allowed_cards_to_play(&trick, &hand);
        assert_eq!(allowed, hand);
    }

    #[rstest]
    fn allowed_cards_must_overtrump_when_cutting(
        #[values(
            // A higher trump is held: only higher trumps are allowed
            (Vec::from([Card::new(5, CardSuits::Trumps), Card::new(12, CardSuits::Trumps), Card::new(15, CardSuits::Trumps), Card::new(3, CardSuits::Hearts)]),
             Vec::from([Card::new(12, CardSuits::Trumps), Card::new(15, CardSuits::Trumps)])),
            // No higher trump: any trump is allowed (under-trumping)
            (Vec::from([Card::new(5, CardSuits::Trumps), Card::new(7, CardSuits::Trumps), Card::new(3, CardSuits::Hearts)]),
             Vec::from([Card::new(5, CardSuits::Trumps), Card::new(7, CardSuits::Trumps)])),
        )]
        case: (Vec<Card>, Vec<Card>),
    ) {
        let (hand, expected) = case;
        // Clubs led, then the previous player cut with the 10 of trumps
        let trick = Trick {
            played_cards: vec![
                Card::new(8, CardSuits::Clubs),
                Card::new(10, CardSuits::Trumps),
            ],
            ..Default::default()
        };
        assert_eq!(allowed_cards_to_play(&trick, &hand), expected);
    }

    #[test]
    fn bot_play_plays_fool_before_the_last_trick() {
        let mut trick = Trick::default();
        let mut hand = vec![
            Card::new(22, CardSuits::Trumps),
            Card::new(5, CardSuits::Spades),
        ];
        trick.bot_play(&mut hand);
        assert_eq!(trick.played_cards, vec![Card::new(22, CardSuits::Trumps)]);
    }
}
