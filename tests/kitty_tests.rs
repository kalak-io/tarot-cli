#[cfg(test)]
mod kitty {
    use tarot_cli::common::{
        card::{Card, CardGetters, CardSuits, KING_RANK},
        kitty::{Kitty, KittyActions},
    };

    // Kitty::new

    #[test]
    fn kitty_new_four_players_has_max_size_six() {
        assert_eq!(Kitty::new(4).max_size, 6);
    }

    #[test]
    fn kitty_new_five_players_has_max_size_three() {
        assert_eq!(Kitty::new(5).max_size, 3);
    }

    #[test]
    fn kitty_new_six_players_has_max_size_three() {
        assert_eq!(Kitty::new(6).max_size, 3);
    }

    // Kitty::bot_compose

    fn plain_cards(n: usize) -> Vec<Card> {
        (2u8..)
            .map(|r| Card::new(r, CardSuits::Hearts))
            .take(n)
            .collect()
    }

    #[test]
    fn bot_compose_fills_kitty_to_max_size() {
        let mut kitty = Kitty::new(4);
        let mut hand: Vec<Card> = plain_cards(10);
        kitty.bot_compose(&mut hand);
        assert_eq!(kitty.cards.len(), 6);
    }

    #[test]
    fn bot_compose_removes_selected_cards_from_hand() {
        let mut kitty = Kitty::new(4);
        let mut hand: Vec<Card> = plain_cards(10);
        let original_len = hand.len();
        kitty.bot_compose(&mut hand);
        assert_eq!(hand.len(), original_len - kitty.max_size);
    }

    #[test]
    fn bot_compose_never_includes_oudlers() {
        let mut kitty = Kitty::new(4);
        let fool = Card::new(22, CardSuits::Trumps);
        let little = Card::new(1, CardSuits::Trumps);
        let big = Card::new(21, CardSuits::Trumps);
        let mut hand: Vec<Card> = plain_cards(10);
        hand.push(fool);
        hand.push(little);
        hand.push(big);

        kitty.bot_compose(&mut hand);

        for card in &kitty.cards {
            assert!(!card.is_oudler(), "Oudler {:?} must not appear in kitty", card);
        }
    }

    #[test]
    fn bot_compose_never_includes_non_trump_kings() {
        let king_h = Card::new(KING_RANK, CardSuits::Hearts);
        let king_c = Card::new(KING_RANK, CardSuits::Clubs);
        let mut hand: Vec<Card> = plain_cards(10);
        hand.push(king_h);
        hand.push(king_c);

        let mut kitty = Kitty::new(4);
        kitty.bot_compose(&mut hand);

        for card in &kitty.cards {
            assert!(
                !(card.rank == KING_RANK && card.suit.name != CardSuits::Trumps),
                "Non-trump King {:?} must not appear in kitty",
                card
            );
        }
    }

    #[test]
    fn bot_compose_prefers_non_trumps_over_trumps() {
        let mut kitty = Kitty::new(4);
        // Mix of cheap non-trumps and cheap trumps — non-trumps should fill the kitty first
        let mut hand: Vec<Card> = (2u8..=9).map(|r| Card::new(r, CardSuits::Hearts)).collect(); // 8 non-trumps
        hand.extend((2u8..=9).map(|r| Card::new(r, CardSuits::Trumps))); // 8 trumps

        kitty.bot_compose(&mut hand);

        let trumps_in_kitty = kitty
            .cards
            .iter()
            .filter(|c| c.suit.name == CardSuits::Trumps)
            .count();
        assert_eq!(trumps_in_kitty, 0, "Kitty should contain no trumps when non-trumps are available");
    }

    #[test]
    fn bot_compose_picks_cheapest_non_trump_cards_first() {
        let mut kitty = Kitty::new(4);
        // Ranks 2–9 of Hearts (all score 0.5) plus a Jack (1.5) and Queen (3.5)
        let mut hand: Vec<Card> = (2u8..=9).map(|r| Card::new(r, CardSuits::Hearts)).collect();
        hand.push(Card::new(11, CardSuits::Hearts)); // Jack
        hand.push(Card::new(13, CardSuits::Hearts)); // Queen

        kitty.bot_compose(&mut hand);

        // All 6 kitty slots should be plain (0.5 pt) cards — not the Jack or Queen
        for card in &kitty.cards {
            assert_eq!(card.score(), 0.5, "Kitty should contain only cheap cards, got {:?}", card);
        }
    }
}
