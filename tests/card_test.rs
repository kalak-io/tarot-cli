mod card {
    use tarot_cli::common::card::{Card, CardActions, CardSuits, Suit};

    #[test]
    fn trump_is_superior_than_suit() {
        let trump = Card::new(5, CardSuits::Trumps);
        let suit = Card::new(6, CardSuits::Hearts);
        assert_eq!(trump.is_superior_than(&suit), true);
    }

    #[test]
    fn trump_is_not_superior_than_other_trump() {
        let trump_1 = Card::new(5, CardSuits::Trumps);
        let trump_2 = Card::new(6, CardSuits::Trumps);
        assert_eq!(trump_1.is_superior_than(&trump_2), false);
    }

    #[test]
    fn trump_is_superior_than_other_trump() {
        let trump_1 = Card::new(6, CardSuits::Trumps);
        let trump_2 = Card::new(5, CardSuits::Trumps);
        assert_eq!(trump_1.is_superior_than(&trump_2), true);
    }

    #[test]
    fn suit_is_not_superior_than_trump() {
        let suit = Card::new(10, CardSuits::Hearts);
        let trump = Card::new(6, CardSuits::Trumps);
        assert_eq!(suit.is_superior_than(&trump), false);
    }

    #[test]
    fn suit_is_superior_than_other_suit() {
        let suit_1 = Card::new(10, CardSuits::Hearts);
        let suit_2 = Card::new(9, CardSuits::Hearts);
        assert_eq!(suit_1.is_superior_than(&suit_2), true);
    }

    #[test]
    fn suit_is_not_superior_than_other_suit() {
        let suit_1 = Card::new(9, CardSuits::Hearts);
        let suit_2 = Card::new(10, CardSuits::Hearts);
        assert_eq!(suit_1.is_superior_than(&suit_2), false);
    }
}
