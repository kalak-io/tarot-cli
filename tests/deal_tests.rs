#[cfg(test)]
mod deal {
    use tarot_cli::common::{deal::Deal, game::Game, kitty::get_max_size_kitty};

    #[test]
    fn deals_right_number_of_cards_with_4_players() {
        let mut game = Game::default();
        let deal = Deal::new(&mut game.players, &mut game.deck);

        let n_cards = deal
            .players
            .iter()
            .fold(0, |acc, player| acc + player.hand.cards.len())
            + deal.kitty.cards.len();
        assert_eq!(n_cards, 78);
        assert_eq!(deal.kitty.cards.len(), 6);
        for player in deal.players {
            assert_eq!(player.hand.cards.len(), 18);
        }
    }

    #[test]
    fn deals_right_number_of_cards_with_5_players() {
        let mut game = Game::new(5);
        let deal = Deal::new(&mut game.players, &mut game.deck);

        let n_cards = deal
            .players
            .iter()
            .fold(0, |acc, player| acc + player.hand.cards.len())
            + deal.kitty.cards.len();
        assert_eq!(n_cards, 78);
        assert_eq!(deal.kitty.cards.len(), 3);
        for player in deal.players {
            assert_eq!(player.hand.cards.len(), 15);
        }
    }

    #[test]
    fn get_max_size_kitty_computes_correctly() {
        assert_eq!(get_max_size_kitty(1), 0);
        assert_eq!(get_max_size_kitty(2), 6);
        assert_eq!(get_max_size_kitty(3), 6);
        assert_eq!(get_max_size_kitty(4), 6);
        assert_eq!(get_max_size_kitty(5), 3);
        assert_eq!(get_max_size_kitty(6), 3);
        assert_eq!(get_max_size_kitty(7), 3);
    }
}
