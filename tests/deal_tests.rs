#[cfg(test)]
mod deal {
    use rstest::rstest;
    use tarot_cli::common::{
        bid::Bids,
        card::{Card, CardSuits},
        chelem::{Chelem, ChelemState},
        deal::{Deal, DealActions, DealGetters},
        game::Game,
        hand::Side,
        kitty::Kitty,
        player::{Player, PlayerKind},
        taker::Taker,
        trick::Trick,
    };

    /// Four players with ids 1 to 4. The player at `taker_index` is the taker.
    fn deal_with_taker(kinds: [PlayerKind; 4], taker_index: usize, bid: Bids) -> Deal {
        let mut players: Vec<Player> = (1..=4)
            .map(|id| Player::new(format!("Player {id}"), id, Some(kinds[id as usize - 1])))
            .collect();
        players[taker_index].hand.side = Side::Attack;
        Deal {
            taker: Some(Taker {
                player: players[taker_index].clone(),
                bid,
            }),
            players,
            ..Default::default()
        }
    }

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
    #[rstest]
    fn test_bonus_petit_au_bout(
        #[values(
            (Vec::from([Trick {
                played_cards: vec![Card::new(1, CardSuits::Trumps)],
                winner_side: Side::Attack,
            }]), Some(Side::Attack)),
            (Vec::from([Trick {
                played_cards: vec![Card::new(1, CardSuits::Trumps)],
                winner_side: Side::Defense,
            }]), Some(Side::Defense)),
            (Vec::from([Trick {
                played_cards: vec![Card::new(1, CardSuits::Hearts)],
                winner_side: Side::Attack,
            }]), None),
        )]
        case: (Vec<Trick>, Option<Side>),
    ) {
        let (tricks, expected) = case;
        let deal = Deal {
            tricks,
            ..Default::default()
        };
        assert_eq!(deal.bonus_petit_au_bout(), expected);
    }

    // "Le Chien et l'Ecart": the kitty counts for the taker, except on a Guard Against
    #[rstest]
    fn set_score_counts_kitty_for_attack_except_on_guard_against(
        #[values(
            // 50 won + 6 in the kitty = 56, exactly the 0-oudler target: 25 × multiplier × 3 defenders
            (Bids::Take, 75.0),
            (Bids::Guard, 150.0),
            (Bids::GuardWithout, 300.0),
            // Without the kitty the taker is 6 short: -(25 + 6) × 6 × 3 defenders
            (Bids::GuardAgainst, -558.0),
        )]
        case: (Bids, f64),
    ) {
        let (bid, expected_taker_score) = case;
        let mut deal = deal_with_taker([PlayerKind::Bot; 4], 0, bid);
        // 100 low cards at 0.5 point each = 50 points, no oudler
        deal.players[0].hand.won_cards = vec![Card::new(2, CardSuits::Clubs); 100];
        // King (4.5) + three low cards (1.5) = 6 points
        deal.kitty = Kitty {
            cards: vec![
                Card::new(14, CardSuits::Hearts),
                Card::new(2, CardSuits::Hearts),
                Card::new(3, CardSuits::Hearts),
                Card::new(4, CardSuits::Hearts),
            ],
            max_size: 6,
        };

        deal.set_score();
        assert_eq!(deal.players[0].score(), expected_taker_score);
        for defender in &deal.players[1..] {
            assert_eq!(defender.score(), -expected_taker_score / 3.0);
        }
    }

    #[test]
    fn take_chelem_records_the_declaration_on_the_taker_in_players() {
        let mut deal = deal_with_taker([PlayerKind::Bot; 4], 2, Bids::Guard);
        deal.take_chelem();
        assert_eq!(
            deal.players[2].hand.bonus_chelem,
            Some(Chelem {
                state: ChelemState::NotAnnounced,
                result: None,
            })
        );
    }

    #[test]
    fn taker_who_announces_chelem_leads_first_trick() {
        let mut kinds = [PlayerKind::Bot; 4];
        kinds[2] = PlayerKind::Human;
        let mut deal = deal_with_taker(kinds, 2, Bids::Guard);
        // A Human hand with a declaration already set is not prompted again
        deal.players[2].hand.bonus_chelem = Some(Chelem {
            state: ChelemState::Announced,
            result: None,
        });

        deal.take_chelem();
        let ids: Vec<u8> = deal.players.iter().map(|p| p.id).collect();
        assert_eq!(ids, vec![3, 4, 1, 2]);
    }
}
