#[cfg(test)]
mod deal {
    use rstest::rstest;
    use tarot_cli::common::{
        bid::Bids,
        card::{Card, CardGetters, CardSuits},
        chelem::{Chelem, ChelemState},
        deal::{Deal, DealActions, DealGetters},
        game::Game,
        hand::{Hand, Side},
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

    // "La distribution": the first and the last cards of the deck never go to the kitty
    #[rstest]
    fn kitty_is_full_and_never_gets_first_or_last_card(
        #[values((4, 6, 18), (5, 3, 15))] case: (u8, usize, usize),
    ) {
        let (n_players, kitty_size, hand_size) = case;
        let mut game = Game::new(n_players);
        for _ in 0..500 {
            let (first_card, last_card) = (game.deck[0], game.deck[77]);
            let deal = Deal::new(&mut game.players, &mut game.deck);
            assert_eq!(deal.kitty.cards.len(), kitty_size);
            assert!(!deal.kitty.cards.contains(&first_card));
            assert!(!deal.kitty.cards.contains(&last_card));
            for player in &deal.players {
                assert_eq!(player.hand.cards.len(), hand_size);
            }
        }
    }

    /// Deal where player 1 attacks alone and every player holds `hands[i]`.
    fn deal_with_hands(hands: [Vec<Card>; 4]) -> Deal {
        let mut deal = deal_with_taker([PlayerKind::Bot; 4], 0, Bids::Take);
        for (player, cards) in deal.players.iter_mut().zip(hands) {
            player.hand = Hand {
                cards,
                side: player.hand.side,
                ..Hand::default()
            };
        }
        deal
    }

    fn side_points(deal: &Deal, side: Side) -> f64 {
        deal.players
            .iter()
            .filter(|p| p.hand.side == side)
            .flat_map(|p| p.hand.won_cards.iter())
            .map(|c| c.score())
            .sum()
    }

    // "Le jeu de la carte": the Fool stays with its side, which gives a low card in exchange
    #[test]
    fn fool_stays_with_its_side_for_a_low_card() {
        let fool = Card::new(22, CardSuits::Trumps);
        let mut deal = deal_with_hands([
            // Player 1 (attack) leads the Fool, then wins trick 2 with the King
            vec![fool, Card::new(14, CardSuits::Hearts)],
            vec![
                Card::new(2, CardSuits::Hearts),
                Card::new(3, CardSuits::Hearts),
            ],
            vec![
                Card::new(5, CardSuits::Hearts),
                Card::new(4, CardSuits::Hearts),
            ],
            vec![
                Card::new(6, CardSuits::Hearts),
                Card::new(7, CardSuits::Hearts),
            ],
        ]);

        deal.play_tricks();

        let winners: Vec<Side> = deal.tricks.iter().map(|t| t.winner_side).collect();
        assert_eq!(winners, vec![Side::Defense, Side::Attack]);
        let taker = deal.players.iter().find(|p| p.id == 1).unwrap();
        assert!(taker.hand.won_cards.contains(&fool));
        // The attack paid its low card once it won trick 2
        assert_eq!(deal.fool_debt, None);
        // Attack: Fool 4.5 + King 4.5 + two low cards. Defense: four low cards.
        assert_eq!(side_points(&deal, Side::Attack), 10.0);
        assert_eq!(side_points(&deal, Side::Defense), 2.0);
    }

    #[rstest]
    fn fool_led_to_last_trick(
        // (winner of the previous trick, id of the player who gets the Fool)
        #[values(
            // No chelem: the Fool goes to the trick winner
            (Side::Defense, 4),
            // The attack won every trick: the Fool wins the last trick
            (Side::Attack, 1),
        )]
        case: (Side, u8),
    ) {
        let (previous_winner, fool_holder) = case;
        let fool = Card::new(22, CardSuits::Trumps);
        let mut deal = deal_with_hands([
            vec![fool],
            vec![Card::new(2, CardSuits::Hearts)],
            vec![Card::new(5, CardSuits::Hearts)],
            vec![Card::new(6, CardSuits::Hearts)],
        ]);
        deal.tricks = vec![Trick {
            winner_side: previous_winner,
            ..Default::default()
        }];

        deal.play_tricks();

        let holder = deal.players.iter().find(|p| p.id == fool_holder).unwrap();
        assert!(holder.hand.won_cards.contains(&fool));
        assert_eq!(holder.hand.won_cards.len(), 4);
    }

    // Each player bids once, and the last bid is the highest
    #[test]
    fn take_bids_gives_the_deal_to_the_highest_bid() {
        let trumps = |ranks: std::ops::RangeInclusive<u8>| -> Vec<Card> {
            ranks
                .map(|rank| Card::new(rank, CardSuits::Trumps))
                .collect()
        };
        let kings: Vec<Card> = [
            CardSuits::Clubs,
            CardSuits::Diamonds,
            CardSuits::Hearts,
            CardSuits::Spades,
        ]
        .into_iter()
        .map(|suit| Card::new(14, suit))
        .collect();
        // Bots bid the same way on the same hand, so duplicate cards across hands are fine
        let mut deal = deal_with_hands([
            // 21 (10) + trumps 2 to 15 (28) = 38: Take
            [trumps(21..=21), trumps(2..=15)].concat(),
            // Fool (8) + trumps 16 to 20 (15) + 4 Kings (24) = 47: Guard
            [trumps(22..=22), trumps(16..=20), kings.clone()].concat(),
            // Little (5) + trumps 2 to 15 (28) + King (6) = 39: a Take, which cannot beat the Guard
            [trumps(1..=1), trumps(2..=15), kings[..1].to_vec()].concat(),
            // Weak hand: Pass
            vec![Card::new(2, CardSuits::Clubs)],
        ]);
        deal.taker = None;

        deal.take_bids();

        let taker = deal.taker.unwrap();
        assert_eq!((taker.player.id, taker.bid), (2, Bids::Guard));
    }

    // "Le jeu à 3 joueurs" and "Le jeu à 5 joueurs": each defender pays the deal score,
    // a partner gets one share and the taker gets the rest
    #[rstest]
    fn set_score_splits_the_deal_score_between_teams(
        // (players, taker has a partner, expected taker score)
        #[values((3, false, 50.0), (4, false, 75.0), (5, true, 50.0), (5, false, 100.0))] case: (
            u8,
            bool,
            f64,
        ),
    ) {
        let (n_players, has_partner, expected_taker_score) = case;
        let mut players: Vec<Player> = (1..=n_players)
            .map(|id| Player::new(format!("Bot {id}"), id, Some(PlayerKind::Bot)))
            .collect();
        players[0].hand.side = Side::Attack;
        if has_partner {
            players[1].hand.side = Side::Attack;
        }
        // 112 low cards = 56 points, exactly the 0-oudler target: the deal is worth 25
        players[0].hand.won_cards = vec![Card::new(2, CardSuits::Clubs); 112];
        let mut deal = Deal {
            taker: Some(Taker {
                player: players[0].clone(),
                bid: Bids::Take,
            }),
            players,
            ..Default::default()
        };

        deal.set_score();

        assert_eq!(deal.players[0].score(), expected_taker_score);
        for player in &deal.players[1..] {
            let expected = if player.hand.side == Side::Attack {
                25.0
            } else {
                -25.0
            };
            assert_eq!(player.score(), expected);
        }
        let total: f64 = deal.players.iter().map(|p| p.score()).sum();
        assert_eq!(total, 0.0);
    }
}
