mod card {
    use tarot_cli::common::card::{Card, CardSuit, CardSuits, CardTrump, Suit};

    #[test]
    fn trump_is_superior_than_suit() {
        let trump = Card::new(CardTrump, 5, Suit::new(CardSuits::Trumps));
        let suit = Card::new(CardSuit, 6, Suit::new(CardSuits::Hearts));
        assert_eq!(trump.is_superior_than(&suit), true);
    }

    #[test]
    fn trump_is_not_superior_than_other_trump() {
        let trump_1 = Card::new(CardTrump, 5, Suit::new(CardSuits::Trumps));
        let trump_2 = Card::new(CardTrump, 6, Suit::new(CardSuits::Trumps));
        assert_eq!(trump_1.is_superior_than(&trump_2), false);
    }

    #[test]
    fn trump_is_superior_than_other_trump() {
        let trump_1 = Card::new(CardTrump, 6, Suit::new(CardSuits::Trumps));
        let trump_2 = Card::new(CardTrump, 5, Suit::new(CardSuits::Trumps));
        assert_eq!(trump_1.is_superior_than(&trump_2), true);
    }

    #[test]
    fn suit_is_not_superior_than_trump() {
        let suit = Card::new(CardSuit, 10, Suit::new(CardSuits::Hearts));
        let trump = Card::new(CardTrump, 6, Suit::new(CardSuits::Trumps));
        assert_eq!(suit.is_superior_than(&trump), false);
    }

    #[test]
    fn suit_is_superior_than_other_suit() {
        let suit_1 = Card::new(CardSuit, 10, Suit::new(CardSuits::Hearts));
        let suit_2 = Card::new(CardSuit, 9, Suit::new(CardSuits::Hearts));
        assert_eq!(suit_1.is_superior_than(&suit_2), true);
    }

    #[test]
    fn suit_is_not_superior_than_other_suit() {
        let suit_1 = Card::new(CardSuit, 9, Suit::new(CardSuits::Hearts));
        let suit_2 = Card::new(CardSuit, 10, Suit::new(CardSuits::Hearts));
        assert_eq!(suit_1.is_superior_than(&suit_2), false);
    }
}
