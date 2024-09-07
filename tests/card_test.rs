mod card {
    use rstest::rstest;
    use tarot_cli::common::card::{Card, CardActions, CardSuits};

    #[rstest]
    fn card_is_superior_than(
        #[values(
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Hearts), Some(CardSuits::Hearts), true),
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Hearts), Some(CardSuits::Clubs), true),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Hearts), Some(CardSuits::Hearts), false),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Clubs), Some(CardSuits::Hearts), true),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Trumps), Some(CardSuits::Clubs), false),
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Trumps), Some(CardSuits::Clubs), false),
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Hearts), None, true),
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Clubs), None, true),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Trumps), None, false),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Clubs), None, false),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Trumps), None, false),
            (Card::new(2, CardSuits::Trumps), Card::new(5, CardSuits::Trumps), None, false),
            (Card::new(2, CardSuits::Hearts), Card::new(5, CardSuits::Hearts), None, false),
        )]
        case: (Card, Card, Option<CardSuits>, bool),
    ) {
        let (card_1, card_2, played_suit, expected) = case;
        assert_eq!(card_1.is_superior_than(&card_2, played_suit), expected);
    }
}
