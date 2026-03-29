#[cfg(test)]
mod score {
    use rstest::rstest;
    use tarot_cli::common::{
        bid::Bids,
        card::{Card, CardSuits, CardSuitsGetters},
        chelem::{Chelem, ChelemResult, ChelemState},
        game::Game,
        hand::{Poignee, Side},
        score::{
            compute_oudlers, compute_points, compute_score, points_chelem, points_petit_au_bout,
            points_poignee, BASE_SCORE,
        },
    };

    #[test]
    fn deck_contains_3_oudlers() {
        let game = Game::default();
        let n_oudlers = compute_oudlers(&game.deck);
        assert_eq!(n_oudlers, 3);
    }

    #[test]
    fn deck_score_equals_91() {
        let game = Game::default();
        assert_eq!(compute_points(&game.deck), 91.0);
    }

    #[test]
    fn trumps_score_equals_23() {
        let game = Game::default();
        let trump_cards = game
            .deck
            .iter()
            .filter(|c| c.suit.name.is_trump())
            .cloned()
            .collect::<Vec<Card>>();
        assert_eq!(compute_points(&trump_cards), 23.0);
    }

    #[test]
    fn suit_score_equals_17() {
        let game = Game::default();
        let mut suits = game
            .deck
            .iter()
            .map(|c| c.suit.name.to_string())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        suits.retain(|suit| *suit != "Trumps");
        for suit in suits {
            let suit_cards = game
                .deck
                .iter()
                .filter(|c| c.suit.name.to_string() == suit)
                .cloned()
                .collect::<Vec<Card>>();
            assert_eq!(compute_points(&suit_cards), 17.0);
        }
    }

    #[rstest]
    fn points_petit_au_bout_returns_correct_value(
        #[values(
            (Some(Side::Attack), 10.0),
            (Some(Side::Defense), -10.0),
            (None, 0.0)
        )]
        case: (Option<Side>, f64),
    ) {
        let (side, expected) = case;
        assert_eq!(points_petit_au_bout(side), expected);
    }

    #[rstest]
    fn points_poignee_returns_correct_value(
        #[values(
            (Some(Poignee::Simple), 20.0),
            (Some(Poignee::Double), 30.0),
            (Some(Poignee::Triple), 40.0),
            (None, 0.0)
        )]
        case: (Option<Poignee>, f64),
    ) {
        let (poignee, expected) = case;
        assert_eq!(points_poignee(poignee), expected);
    }

    #[rstest]
    fn points_chelem_returns_correct_value(
        #[values(
            (Some(Chelem { state: ChelemState::Announced, result: Some(ChelemResult::AnnouncedAndSucceed) }), 400.0),
            (Some(Chelem { state: ChelemState::NotAnnounced, result: Some(ChelemResult::NotAnnouncedAndSucceed) }), 200.0),
            (Some(Chelem { state: ChelemState::Announced, result: Some(ChelemResult::AnnouncedAndLost) }), -200.0),
            (None, 0.0)
        )]
        case: (Option<Chelem>, f64),
    ) {
        let (chelem, expected) = case;
        assert_eq!(points_chelem(chelem), expected);
    }

    // Cards that score exactly on the 3-oudler threshold (36 pts) so diff=0.
    // Used to isolate the multiplier: final score = (BASE_SCORE + 0) × multiplier.
    fn threshold_cards() -> Vec<Card> {
        [
            Card::new(22, CardSuits::Trumps), // Fool  (4.5)
            Card::new(1, CardSuits::Trumps),  // Little (4.5)
            Card::new(21, CardSuits::Trumps), // Big    (4.5)
        ]
        .into_iter()
        .chain(
            (1..=9u8).flat_map(|r| {
                [
                    Card::new(r, CardSuits::Hearts),
                    Card::new(r, CardSuits::Clubs),
                    Card::new(r, CardSuits::Spades),
                    Card::new(r, CardSuits::Diamonds),
                ]
            }),
        )
        .chain((1..=9u8).map(|r| Card::new(r, CardSuits::Hearts)))
        .take(48) // 3 oudlers (13.5) + 45 plain (22.5) = 36.0 pts exactly
        .collect()
    }

    #[rstest]
    fn compute_score_applies_correct_bid_multiplier(
        #[values(
            (Bids::Take,         BASE_SCORE * 1.0),
            (Bids::Guard,        BASE_SCORE * 2.0),
            (Bids::GuardWithout, BASE_SCORE * 4.0),
            (Bids::GuardAgainst, BASE_SCORE * 6.0),
        )]
        case: (Bids, f64),
    ) {
        let (bid, expected) = case;
        let cards = threshold_cards();
        assert_eq!(compute_score(&cards, &bid, None, None, None), expected);
    }

    #[test]
    fn compute_score_includes_poignee_and_chelem_bonuses() {
        // 3 oudlers (Fool=22, Little=1, Big=21) + 45 plain cards (0.5 pt each)
        // Points: 3×4.5 + 45×0.5 = 13.5 + 22.5 = 36.0 (exactly the 3-oudler threshold)
        // diff = 36.0 - 36.0 = 0
        let cards: Vec<Card> = [
            Card::new(22, CardSuits::Trumps), // Fool  (oudler)
            Card::new(1, CardSuits::Trumps),  // Little (oudler)
            Card::new(21, CardSuits::Trumps), // Big    (oudler)
        ]
        .into_iter()
        // 45 plain non-trump cards (rank 1–9 of each suit = 9×4 = 36, plus 9 more)
        .chain((1..=9u8).flat_map(|r| {
            [
                Card::new(r, CardSuits::Hearts),
                Card::new(r, CardSuits::Clubs),
                Card::new(r, CardSuits::Spades),
                Card::new(r, CardSuits::Diamonds),
            ]
        }))
        .chain((1..=9u8).map(|r| Card::new(r, CardSuits::Hearts))) // 9 extra
        .take(48) // 3 oudlers + 45 plain = 48 cards, 36.0 pts
        .collect();

        // diff = 0, bid Take (×1): base = (25 + 0) × 1 = 25
        // + Poignee::Simple (20) + AnnouncedAndSucceed (400) = 445
        let score = compute_score(
            &cards,
            &Bids::Take,
            None,
            Some(Chelem {
                state: ChelemState::Announced,
                result: Some(ChelemResult::AnnouncedAndSucceed),
            }),
            Some(Poignee::Simple),
        );
        assert_eq!(score, 445.0);
    }
}
