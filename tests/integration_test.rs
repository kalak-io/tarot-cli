#[cfg(test)]
mod integration {
    use rstest::rstest;
    use tarot_cli::common::{
        deal::{Deal, DealActions},
        game::{Game, GameActions, ReorderBy},
    };

    const SEED: u64 = 42;

    /// Deal until a bot bids, then play the deal to the end and score it
    fn play_one_deal(game: &mut Game) -> Deal {
        let mut deal = loop {
            game.shuffle_deck();
            game.split_deck();
            game.update_dealer();
            game.reorder_players(ReorderBy::Dealer);

            let mut deal = Deal::new(&mut game.players, &mut game.deck, &mut game.rng);
            deal.take_bids();
            if deal.taker.is_some() {
                break deal;
            }
        };
        deal.call_king(); // 5-player games only
        deal.set_side();
        deal.compose_kitty();
        deal.take_chelem();
        deal.play_tricks();
        deal.set_score();
        deal
    }

    /// Run one complete deal (setup → bids → kitty → tricks → score) with
    /// four bots and verify the key invariants of the game:
    ///
    /// 1. Every card in the hands is played: 24, 18 or 15 tricks for 3, 4 or 5 players.
    /// 2. All 78 cards are accounted for (won_cards + kitty).
    /// 3. Scores are zero-sum across all players.
    #[rstest]
    fn full_deal_completes_and_preserves_invariants(
        #[values((3, 24), (4, 18), (5, 15))] case: (u8, usize),
    ) {
        let (n_players, expected_tricks) = case;
        let mut game = Game::new_bots(n_players, SEED);
        let deal = play_one_deal(&mut game);
        deal.show_score();

        // 1. Every card in the hands is played
        assert_eq!(
            deal.tricks.len(),
            expected_tricks,
            "wrong number of tricks for a {n_players}-player deal"
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
    /// 2. N_PLAYERS deals are played.
    /// 3. Cumulative scores across all deals remain zero-sum.
    /// 4. `game.players` carries each player's running total.
    ///
    /// Note: `game.deck` is left untouched by `draw_cards` (which takes an
    /// immutable slice), so no `collect_deck` call is needed between deals.
    #[test]
    fn game_round_rotates_dealer_and_scores_are_zero_sum() {
        const N_PLAYERS: u8 = 4;
        let mut game = Game::new_bots(N_PLAYERS, SEED);

        let mut dealer_ids: Vec<u8> = Vec::new();
        let mut n_deals = 0;

        for _ in 0..N_PLAYERS {
            game.shuffle_deck();
            game.split_deck();
            game.update_dealer();
            game.reorder_players(ReorderBy::Dealer);

            // Record who is dealer this round
            let dealer_id = game.players.iter().find(|p| p.is_dealer()).unwrap().id;
            dealer_ids.push(dealer_id);

            // Retry until at least one bot bids
            // (game.deck is never modified by draw_cards, so retries are free)
            let mut deal = loop {
                game.shuffle_deck();
                game.split_deck();
                let mut d = Deal::new(&mut game.players, &mut game.deck, &mut game.rng);
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

            n_deals += 1;
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
        assert_eq!(n_deals, N_PLAYERS as usize);

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

    /// The player count prompt asks again after an invalid answer.
    /// With a valid count, the game starts, then ends when stdin closes.
    #[test]
    fn player_count_prompt_asks_again_after_invalid_input() {
        use std::io::Write;
        let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_tarot-cli"))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        // Closing stdin after these lines ends the game at its next prompt
        child.stdin.take().unwrap().write_all(b"9\n5\n").unwrap();
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(output.status.success(), "exit status: {}", output.status);
        assert!(stdout.contains("Please enter a number from 3 to 5."));
        assert!(stdout.contains("The dealer is"));
        assert!(stdout.contains("Input closed"));
    }

    /// The same seed plays the same deal: same cards in each trick, same scores
    #[rstest]
    fn same_seed_plays_the_same_deal(#[values(3, 4, 5)] n_players: u8) {
        let first = play_one_deal(&mut Game::new_bots(n_players, SEED));
        let second = play_one_deal(&mut Game::new_bots(n_players, SEED));

        let played = |deal: &Deal| -> Vec<Vec<String>> {
            deal.tricks
                .iter()
                .map(|t| t.played_cards.iter().map(|c| c.to_string()).collect())
                .collect()
        };
        let scores = |deal: &Deal| -> Vec<(u8, f64)> {
            deal.players.iter().map(|p| (p.id, p.score())).collect()
        };
        assert_eq!(played(&first), played(&second));
        assert_eq!(scores(&first), scores(&second));
    }

    #[test]
    fn different_seeds_shuffle_different_decks() {
        assert_ne!(Game::new_bots(4, 1).deck, Game::new_bots(4, 2).deck);
    }
}
