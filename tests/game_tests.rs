#[cfg(test)]
mod game {
    use tarot_cli::common::{
        card::{Card, CardGetters, CardSuits},
        game::{find_dealer, Game, GameActions},
        utils::get_next_index,
    };

    #[test]
    fn deck_has_78_cards() {
        let game = Game::default();
        assert_eq!(game.deck.len(), 78);
    }

    #[test]
    fn deck_contains_22_trump_cards() {
        let game = Game::default();
        let trump_cards = game
            .deck
            .iter()
            .filter(|c| c.is_trump())
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(trump_cards, 22);
    }

    #[test]
    fn deck_contains_5_different_suits() {
        let game = Game::default();
        let mut suits = game
            .deck
            .iter()
            .map(|c| c.suit.name.to_string())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        assert_eq!(suits.len(), 5);
    }

    #[test]
    fn deck_contains_14_cards_by_suit() {
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
            assert_eq!(
                game.deck
                    .iter()
                    .filter(|c| c.suit.name.to_string() == suit)
                    .count(),
                14
            );
        }
    }

    #[test]
    fn split_deck_conserves_original_size_of_deck() {
        let mut game = Game::default();
        let original_len = game.deck.len();
        game.split_deck();
        assert_eq!(original_len, game.deck.len());
    }

    #[test]
    fn split_deck_changes_the_order_of_cards() {
        let mut game = Game::default();
        let fisrt_card = game.deck.first().cloned().unwrap();
        let last_card = game.deck.last().cloned().unwrap();
        game.split_deck();
        assert_ne!(fisrt_card.id(), game.deck.first().unwrap().id());
        assert_ne!(last_card.id(), game.deck.last().unwrap().id());
    }

    #[test]
    fn create_the_right_number_of_player_default() {
        let game = Game::default();
        assert_eq!(game.players.len(), 4);
    }

    #[test]
    fn create_the_expected_number_of_players() {
        let game = Game::new(5);
        assert_eq!(game.players.len(), 5);
    }

    #[test]
    fn set_only_one_dealer() {
        let mut game = Game::default();
        game.update_dealer();
        let n_dealer = game.players.iter().filter(|p| p.is_dealer).count();
        assert_eq!(n_dealer, 1);
    }

    #[test]
    fn update_dealer_is_next_player() {
        let mut game = Game::default();
        let current_dealer = find_dealer(&game.players);
        let next_dealer = get_next_index(&game.players, current_dealer);
        assert_eq!(game.players[current_dealer].is_dealer, true);
        game.update_dealer();
        assert_eq!(game.players[next_dealer].is_dealer, true);
    }
}
