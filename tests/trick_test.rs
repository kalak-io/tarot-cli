#[cfg(test)]
mod trick {
    use rstest::rstest;
    use tarot_cli::common::{
        card::{Card, CardSuits},
        trick::{check_selected_card, Trick, TrickActions},
    };

    #[rstest]
    fn trick_get_best_played_card_index(
        #[values(
            (Vec::new(), None),
            (Vec::from([Card::new(2, CardSuits::Clubs), Card::new(14, CardSuits::Clubs)]), Some(1)),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Some(1)),
            (Vec::from([Card::new(8, CardSuits::Clubs), Card::new(10, CardSuits::Clubs), Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Clubs), Card::new(1, CardSuits::Clubs)]), Some(2)),
            (Vec::from([Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs)]), Some(0)))]
        case: (Vec<Card>, Option<usize>),
    ) {
        let (played_cards, expected_index) = case;
        let mut trick = Trick::default();
        for played_card in played_cards {
            trick.played_cards.push(played_card);
        }
        assert_eq!(trick.get_best_played_card_index(), expected_index);
    }

    #[rstest]
    fn check_selected_card_in_his_context(
        #[values(
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), Some(CardSuits::Clubs), false),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), None, true),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), Some(CardSuits::Trumps), false),
            (Vec::from([Card::new(14, CardSuits::Clubs), Card::new(2, CardSuits::Trumps), Card::new(2, CardSuits::Clubs), Card::new(2, CardSuits::Hearts)]), Card::new(14, CardSuits::Hearts), Some(CardSuits::Trumps), false))]
        case: (Vec<Card>, Card, Option<CardSuits>, bool),
    ) {
        let (cards, selected_card, played_suit, expected_result) = case;
        assert_eq!(
            check_selected_card(&cards, &selected_card, played_suit),
            expected_result
        );
    }
}
