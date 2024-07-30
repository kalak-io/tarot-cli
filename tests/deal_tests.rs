#[cfg(test)]
mod deal {
    use tarot_cli::common::{
        deal::{get_kitty_expected_size, Deal},
        game::Game,
    };

    #[test]
    fn deals_right_number_of_cards_with_4_players() {
        let mut game = Game::default();
        let deal = Deal::new(&mut game.players, &mut game.deck);

        let n_cards = deal
            .players
            .iter()
            .fold(0, |acc, player| acc + player.cards.len())
            + deal.kitty.len();
        assert_eq!(n_cards, 78);
        assert_eq!(deal.kitty.len(), 6);
        for player in deal.players {
            assert_eq!(player.cards.len(), 18);
        }
    }

    #[test]
    fn deals_right_number_of_cards_with_5_players() {
        let mut game = Game::new(5);
        let deal = Deal::new(&mut game.players, &mut game.deck);

        let n_cards = deal
            .players
            .iter()
            .fold(0, |acc, player| acc + player.cards.len())
            + deal.kitty.len();
        assert_eq!(n_cards, 78);
        assert_eq!(deal.kitty.len(), 3);
        for player in deal.players {
            assert_eq!(player.cards.len(), 15);
        }
    }

    #[test]
    fn get_kitty_expected_size_computes_correctly() {
        assert_eq!(get_kitty_expected_size(1), 0);
        assert_eq!(get_kitty_expected_size(2), 6);
        assert_eq!(get_kitty_expected_size(3), 6);
        assert_eq!(get_kitty_expected_size(4), 6);
        assert_eq!(get_kitty_expected_size(5), 3);
        assert_eq!(get_kitty_expected_size(6), 3);
        assert_eq!(get_kitty_expected_size(7), 3);
    }
}
