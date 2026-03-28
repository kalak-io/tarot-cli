#[cfg(test)]
mod integration {
    use tarot_cli::common::{
        deal::{Deal, DealActions},
        game::{create_deck, Game, GameActions, ReorderBy},
        player::{Player, PlayerKind},
    };

    /// Build a 4-player all-bot game without going through `Game::new`,
    /// which always creates Player 1 as a Human (blocking stdin).
    fn create_all_bot_game(n_players: u8) -> Game {
        let mut players: Vec<Player> = (1..=n_players)
            .map(|i| Player::new(format!("Bot {i}"), i, Some(PlayerKind::Bot)))
            .collect();
        players[0].toggle_role(); // designate an initial dealer
        Game {
            players,
            deck: create_deck(),
            deals: Vec::new(),
        }
    }

    /// Run one complete deal (setup → bids → kitty → tricks → score) with
    /// four bots and verify the key invariants of the game:
    ///
    /// 1. Exactly 18 tricks are played (72 cards / 4 players).
    /// 2. All 78 cards are accounted for (won_cards + kitty).
    /// 3. Scores are zero-sum across all players.
    #[test]
    fn full_deal_completes_and_preserves_invariants() {
        let mut game = create_all_bot_game(4);

        // Retry until at least one bot bids — with random hands some deals
        // may produce an all-pass result, which is valid but unplayable.
        let mut deal = loop {
            game.split_deck();
            game.update_dealer();
            game.reorder_players(ReorderBy::Dealer);

            let mut deal = Deal::new(&mut game.players, &mut game.deck);
            deal.take_bids();

            if deal.taker.is_some() {
                break deal;
            }
            // No taker: cards are still in player hands; deck is untouched.
            // Simply retry with a fresh shuffle on the same deck.
        };

        deal.call_king(); // no-op for 4-player games
        deal.set_side();
        deal.compose_kitty();
        deal.take_chelem();
        game.reorder_players(ReorderBy::Chelem); // no-op if no chelem announced
        deal.play_tricks();
        deal.set_score();
        deal.show_score();

        // 1. Correct number of tricks (78 − 6 kitty cards) / 4 players = 18
        assert_eq!(deal.tricks.len(), 18, "expected 18 tricks for a 4-player deal");

        // 2. All 78 cards are accounted for
        let total_won: usize = deal
            .players
            .iter()
            .map(|p| p.hand.won_cards.len())
            .sum();
        assert_eq!(
            total_won + deal.kitty.cards.len(),
            78,
            "total cards in won_cards + kitty must equal 78"
        );

        // 3. Zero-sum: taker gains what defenders lose (and vice-versa)
        let total_score: f64 = deal.players.iter().map(|p| p.score()).sum();
        assert!(
            total_score.abs() < 1e-9,
            "scores must sum to zero, got {total_score}"
        );
    }
}
