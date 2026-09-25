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
        deal.play_tricks();
        deal.set_score();
        deal.show_score();

        // 1. Correct number of tricks (78 − 6 kitty cards) / 4 players = 18
        assert_eq!(
            deal.tricks.len(),
            18,
            "expected 18 tricks for a 4-player deal"
        );

        // 2. All 78 cards are accounted for
        let total_won: usize = deal.players.iter().map(|p| p.hand.won_cards.len()).sum();
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

    /// Simulate a full game round: N_PLAYERS deals so that every player
    /// acts as dealer exactly once.
    ///
    /// Verified invariants:
    /// 1. Dealer rotates through all players (each deals once).
    /// 2. N_PLAYERS deals are stored in game.deals.
    /// 3. Cumulative scores across all deals remain zero-sum.
    /// 4. `game.players` carries each player's running total.
    ///
    /// Note: `game.deck` is left untouched by `draw_cards` (which takes an
    /// immutable slice), so no `collect_deck` call is needed between deals.
    #[test]
    fn game_round_rotates_dealer_and_scores_are_zero_sum() {
        const N_PLAYERS: u8 = 4;
        let mut game = create_all_bot_game(N_PLAYERS);

        let mut dealer_ids: Vec<u8> = Vec::new();

        for _ in 0..N_PLAYERS {
            game.split_deck();
            game.update_dealer();
            game.reorder_players(ReorderBy::Dealer);

            // Record who is dealer this round
            let dealer_id = game.players.iter().find(|p| p.is_dealer()).unwrap().id;
            dealer_ids.push(dealer_id);

            // Retry until at least one bot bids
            // (game.deck is never modified by draw_cards, so retries are free)
            let mut deal = loop {
                let mut d = Deal::new(&mut game.players, &mut game.deck);
                d.take_bids();
                if d.taker.is_some() {
                    break d;
                }
            };

            deal.call_king();
            deal.set_side();
            deal.compose_kitty();
            deal.take_chelem();
            deal.play_tricks();
            deal.set_score();
            game.update_scores(&deal.players);

            // 4. Deal players start from the game totals, so both hold the same total
            for player in &deal.players {
                let game_player = game.players.iter().find(|p| p.id == player.id).unwrap();
                assert_eq!(game_player.score(), player.score());
            }

            game.collect_deck(&deal.players, &deal.kitty.cards);

            // After collection the deck must always be back to 78 cards
            assert_eq!(
                game.deck.len(),
                78,
                "deck must contain 78 cards after collect_deck"
            );

            game.deals.push(deal);
        }

        // 1. Every player was dealer exactly once
        let mut unique_dealers = dealer_ids.clone();
        unique_dealers.sort_unstable();
        unique_dealers.dedup();
        assert_eq!(
            unique_dealers.len(),
            N_PLAYERS as usize,
            "each player must deal once; saw dealer sequence {dealer_ids:?}"
        );

        // 2. Correct number of deals recorded
        assert_eq!(game.deals.len(), N_PLAYERS as usize);

        // 3. Zero-sum across all deals
        let grand_total: f64 = game.players.iter().map(|p| p.score()).sum();
        assert!(
            grand_total.abs() < 1e-9,
            "cumulative scores must sum to zero, got {grand_total}"
        );
    }

    /// Closing stdin (Ctrl-D) ends the game cleanly instead of looping or overflowing the stack.
    #[test]
    fn game_exits_cleanly_when_stdin_closes() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_tarot-cli"))
            .stdin(std::process::Stdio::null())
            .output()
            .unwrap();
        assert!(output.status.success(), "exit status: {}", output.status);
        assert!(String::from_utf8_lossy(&output.stdout).contains("Input closed"));
    }
}
