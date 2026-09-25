#[cfg(test)]
mod game {
    use tarot_cli::common::{
        card::{Card, CardGetters, CardSuitsGetters},
        game::{find_dealer, parse_player_count, Game, GameActions},
        player::PlayerActions,
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
            .filter(|c| c.suit.name.is_trump())
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
    fn shuffle_deck_keeps_the_same_cards_in_a_new_order() {
        let mut game = Game::new_bots(4, 42);
        let before: Vec<String> = game.deck.iter().map(|c| c.id()).collect();
        game.shuffle_deck();
        let after: Vec<String> = game.deck.iter().map(|c| c.id()).collect();
        assert_ne!(before, after);
        let (mut before, mut after) = (before, after);
        before.sort();
        after.sort();
        assert_eq!(before, after);
    }

    #[test]
    fn split_deck_changes_the_order_of_cards() {
        let mut game = Game::default();
        let first_card = game.deck.first().cloned().unwrap();
        let last_card = game.deck.last().cloned().unwrap();
        game.split_deck();
        assert_ne!(first_card.id(), game.deck.first().unwrap().id());
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
        let n_dealer = game
            .players
            .iter()
            .filter(|player| player.is_dealer())
            .count();
        assert_eq!(n_dealer, 1);
    }

    #[test]
    fn update_dealer_is_next_player() {
        let mut game = Game::default();
        let current_dealer = find_dealer(&game.players);
        let next_dealer = get_next_index(&game.players, current_dealer);
        assert!(game.players[current_dealer].is_dealer());
        game.update_dealer();
        assert!(game.players[next_dealer].is_dealer());
    }

    #[test]
    fn update_scores_copies_deal_totals_by_player_id() {
        let mut game = Game::default();
        // Deal players are copies in a different order (tricks reorder them)
        let mut deal_players = game.players.clone();
        deal_players.reverse();
        for player in &mut deal_players {
            player.update_score(player.id as f64 * 10.0);
        }

        game.update_scores(&deal_players);
        for player in &game.players {
            assert_eq!(player.score(), player.id as f64 * 10.0);
        }

        // The next deal starts from the running totals
        for player in &mut deal_players {
            player.update_score(-5.0);
        }
        game.update_scores(&deal_players);
        for player in &game.players {
            assert_eq!(player.score(), player.id as f64 * 10.0 - 5.0);
        }
    }

    // "La distribution": the cut takes or leaves more than 3 cards
    #[test]
    fn split_deck_leaves_more_than_3_cards_in_each_part() {
        let mut game = Game::default();
        for _ in 0..1000 {
            let first_card = game.deck[0];
            game.split_deck();
            // The old first card lands after the part that moved to the top
            let moved = game.deck.iter().position(|c| *c == first_card).unwrap();
            assert!(
                (4..=74).contains(&moved),
                "cut moved {moved} cards to the top"
            );
        }
    }

    #[rstest::rstest]
    fn parse_player_count_accepts_3_to_5_players(
        #[values(
            ("3\n", Some(3)),
            ("4", Some(4)),
            (" 5 \n", Some(5)),
            // Enter alone picks the default
            ("\n", Some(4)),
            ("2\n", None),
            ("6\n", None),
            ("four\n", None),
            ("-1\n", None),
        )]
        case: (&str, Option<u8>),
    ) {
        let (input, expected) = case;
        assert_eq!(parse_player_count(input), expected);
    }
}
