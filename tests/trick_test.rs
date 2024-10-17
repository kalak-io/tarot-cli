#[cfg(test)]
mod trick {
    use rstest::rstest;
    use tarot_cli::common::{
        card::{Card, CardSuits},
        trick::{check_selected_card, Trick, TrickActions, TrickGetters},
    };

    #[rstest]
    fn trick_get_best_played_card_index(
        #[values(
            (Vec::new(), None),
            (Vec::from([Card::new(2, CardSuits::Clubs), Card::new(14, CardSuits::Clubs)]), Some(1)),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Some(1)),
            (Vec::from([Card::new(8, CardSuits::Clubs), Card::new(10, CardSuits::Clubs), Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Clubs), Card::new(1, CardSuits::Clubs)]), Some(2)),
            (Vec::from([Card::new(8, CardSuits::Trumps), Card::new(10, CardSuits::Trumps), Card::new(14, CardSuits::Trumps), Card::new(2, CardSuits::Trumps), Card::new(1, CardSuits::Trumps)]), Some(2)),
            (Vec::from([Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs)]), Some(0)))]
        case: (Vec<Card>, Option<usize>),
    ) {
        let (played_cards, expected_index) = case;
        let mut trick = Trick::default();
        for played_card in played_cards {
            trick.played_cards.push(played_card);
        }
        assert_eq!(trick.get_best_played_card_index(trick.played_suit()), expected_index);
    }

    #[rstest]
    fn check_selected_card_in_his_context(
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
            // (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), None, true),
            // (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), Some(CardSuits::Trumps), false),
            // (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), Some(CardSuits::Trumps), false))]
        case: (Vec<Card>, Vec<Card>, Card, bool),
    ) {
        let (played_cards, player_cards, selected_card, expected_result) = case;
        let mut trick = Trick::default();
        trick.played_cards = played_cards;
        assert_eq!(
            check_selected_card(&trick, &player_cards, &selected_card),
            expected_result
        );
    }
}
