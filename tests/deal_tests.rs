#[cfg(test)]
mod deal {
    use rstest::rstest;
    use tarot_cli::common::{deal::Deal, game::Game};

    #[rstest]
    fn deals_right_number_of_cards(#[values((4, 6, 18), (5, 3, 15))] case: (u8, usize, usize)) {
        let (n_player, expected_kitty_size, expected_n_cards_by_player) = case;
        let mut game = Game::new(n_player);
        let deal = Deal::new(&mut game.players, &mut game.deck);

        let n_cards = deal
            .players
            .iter()
            .fold(0, |acc, player| acc + player.hand.cards.len())
            + deal.kitty.cards.len();
        assert_eq!(n_cards, 78);
        assert_eq!(deal.kitty.cards.len(), expected_kitty_size);
        for player in deal.players {
            assert_eq!(player.hand.cards.len(), expected_n_cards_by_player);
        }
    }

    #[rstest]
    fn get_max_size_kitty_computes_correctly(
        #[values((1, 0), (2, 6), (3, 6), (4, 6), (5, 3), (6, 3), (7, 3))] case: (u8, usize),
    ) {
        let (n_players, expected_max_size) = case;
        let mut game = Game::new(n_players);
        let deal = Deal::new(&mut game.players, &mut game.deck);
        assert_eq!(deal.kitty.max_size, expected_max_size);
    }
}
