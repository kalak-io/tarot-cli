#[cfg(test)]
mod score {
    use tarot_cli::common::{
        card::{Card, CardSuitsGetters},
        game::Game,
        score::{compute_oudlers, compute_points},
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
}
