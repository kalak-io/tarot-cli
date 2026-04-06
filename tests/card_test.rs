mod card {
    use rstest::rstest;
    use tarot_cli::common::card::{count_cards_by_hand, Card, CardActions, CardGetters, CardSuits};

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

    #[rstest]
    fn card_score_returns_correct_value(
        #[values(
            (Card::new(22, CardSuits::Trumps), 4.5),  // Fool (oudler)
            (Card::new(1,  CardSuits::Trumps), 4.5),  // Little (oudler)
            (Card::new(21, CardSuits::Trumps), 4.5),  // Big (oudler)
            (Card::new(5,  CardSuits::Trumps), 0.5),  // plain trump
            (Card::new(14, CardSuits::Hearts), 4.5),  // King
            (Card::new(13, CardSuits::Clubs),  3.5),  // Queen
            (Card::new(12, CardSuits::Spades), 2.5),  // Knight
            (Card::new(11, CardSuits::Diamonds), 1.5), // Jack
            (Card::new(5,  CardSuits::Hearts), 0.5),  // plain suit card
        )]
        case: (Card, f64),
    ) {
        let (card, expected) = case;
        assert_eq!(card.score(), expected);
    }

    #[rstest]
    fn card_is_oudler_returns_correct_value(
        #[values(
            (Card::new(22, CardSuits::Trumps), true),   // Fool
            (Card::new(1,  CardSuits::Trumps), true),   // Little
            (Card::new(21, CardSuits::Trumps), true),   // Big
            (Card::new(14, CardSuits::Trumps), false),  // Trump king rank — not an oudler
            (Card::new(5,  CardSuits::Trumps), false),  // plain trump
            (Card::new(14, CardSuits::Hearts), false),  // King of Hearts
            (Card::new(1,  CardSuits::Clubs),  false),  // Ace of Clubs
        )]
        case: (Card, bool),
    ) {
        let (card, expected) = case;
        assert_eq!(card.is_oudler(), expected);
    }

    #[rstest]
    fn card_name_returns_correct_value(
        #[values(
            (Card::new(22, CardSuits::Trumps), "Fool"),
            (Card::new(14, CardSuits::Hearts), "King"),
            (Card::new(13, CardSuits::Clubs),  "Queen"),
            (Card::new(12, CardSuits::Spades), "Knight"),
            (Card::new(11, CardSuits::Diamonds), "Jack"),
            (Card::new(5,  CardSuits::Hearts), "5"),
            (Card::new(1,  CardSuits::Trumps), "1"),
        )]
        case: (Card, &str),
    ) {
        let (card, expected) = case;
        assert_eq!(card.name(), expected);
    }

    #[rstest]
    fn card_id_returns_correct_value(
        #[values(
            (Card::new(5,  CardSuits::Hearts),   "H5"),
            (Card::new(14, CardSuits::Clubs),    "C14"),
            (Card::new(1,  CardSuits::Trumps),   "T1"),
            (Card::new(22, CardSuits::Trumps),   "T22"),
            (Card::new(3,  CardSuits::Diamonds), "D3"),
            (Card::new(7,  CardSuits::Spades),   "S7"),
        )]
        case: (Card, &str),
    ) {
        let (card, expected) = case;
        assert_eq!(card.id(), expected);
    }

    #[rstest]
    fn count_cards_by_hand_returns_correct_value(
        #[values(
            (2, 18),
            (3, 18),
            (4, 18),
            (5, 15),
            (6, 15),
        )]
        case: (usize, u8),
    ) {
        let (n_players, expected) = case;
        assert_eq!(count_cards_by_hand(n_players), expected);
    }

    #[test]
    fn trump_suit_icon_is_star() {
        let card = Card::new(5, CardSuits::Trumps);
        assert_eq!(card.suit.icon, '★');
    }
}
