mod deck {
    use super::super::*;

    #[test]
    fn created_deck_has_78_cards() {
        let deck = build_deck();
        assert_eq!(deck.len(), 78);
    }

    #[test]
    fn deck_contains_22_trump_cards() {
        let deck = build_deck();
        let trump_cards = deck
            .iter()
            .filter(|c| c.suit.name == "Trumps")
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(trump_cards, 22);
    }

    #[test]
    fn deck_contains_5_different_suits() {
        // Four suits + Trumps
        let deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        assert_eq!(suits.len(), 5);
    }

    #[test]
    fn deck_contains_14_cards_by_suit() {
        let deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        suits.retain(|suit| *suit != "Trumps");
        for suit in suits {
            assert_eq!(deck.iter().filter(|c| c.suit.name == suit).count(), 14);
        }
    }

    #[test]
    fn deck_contains_3_oudlers() {
        let deck = build_deck();
        let n_oudlers = compute_oudlers(&deck);
        assert_eq!(n_oudlers, 3);
    }

    #[test]
    fn deck_score_equals_91() {
        let deck = build_deck();
        assert_eq!(compute_points(&deck), 91.0);
    }

    #[test]
    fn trumps_score_equals_23() {
        let deck = build_deck();
        let trump_cards = deck
            .iter()
            .filter(|c| c.suit.name == "Trumps")
            .cloned()
            .collect::<Vec<Card>>();
        assert_eq!(compute_points(&trump_cards), 23.0);
    }

    #[test]
    fn suit_score_equals_17() {
        let deck = build_deck();
        let mut suits = deck
            .iter()
            .map(|c| c.suit.name.clone())
            .collect::<Vec<String>>();
        suits.sort();
        suits.dedup();
        suits.retain(|suit| *suit != "Trumps");
        for suit in suits {
            let suit_cards = deck
                .iter()
                .filter(|c| c.suit.name == suit)
                .cloned()
                .collect::<Vec<Card>>();
            assert_eq!(compute_points(&suit_cards), 17.0);
        }
    }

    #[test]
    fn split_deck_conserves_original_size_of_deck() {
        let mut deck = build_deck();
        let original_len = deck.len();
        split_deck(&mut deck);
        assert_eq!(original_len, deck.len());
    }

    #[test]
    fn split_deck_changes_the_order_of_cards() {
        let mut deck = build_deck();
        let fisrt_card = deck.first().cloned().unwrap();
        let last_card = deck.last().cloned().unwrap();
        split_deck(&mut deck);
        assert_ne!(fisrt_card.id(), deck.first().unwrap().id());
        assert_ne!(last_card.id(), deck.last().unwrap().id());
    }

    #[test]
    fn deals_right_number_of_cards_with_4_players() {
        let mut deck = build_deck();
        let mut players = create_players(&Config::default());
        let mut kitty = Vec::new();
        deal_cards(&mut deck, &mut players, &mut kitty);
        let n_cards = players
            .iter()
            .fold(0, |acc, player| acc + player.cards.len())
            + kitty.len();
        assert_eq!(n_cards, 78);
        assert_eq!(kitty.len(), 6);
        for player in players {
            assert_eq!(player.cards.len(), 18);
        }
    }

    #[test]
    fn deals_right_number_of_cards_with_5_players() {
        let mut deck = build_deck();
        let args = [String::from("target/debug/tarot-cli"), String::from("5")];
        let config = Config::build(&args).unwrap();
        let mut players = create_players(&config);
        let mut kitty = Vec::new();
        deal_cards(&mut deck, &mut players, &mut kitty);
        let n_cards = players
            .iter()
            .fold(0, |acc, player| acc + player.cards.len())
            + kitty.len();
        assert_eq!(n_cards, 78);
        assert_eq!(kitty.len(), 3);
        for player in players {
            assert_eq!(player.cards.len(), 15);
        }
    }
}

#[cfg(test)]
mod kitty {
    use super::super::*;

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

#[cfg(test)]
mod players {
    use super::super::*;
    const MAX_N_PLAYERS: u8 = 5;

    #[test]
    fn create_the_right_number_of_players_default() {
        let args = [String::from("target/debug/tarot-cli")];
        let config = Config::build(&args).unwrap();
        let players = create_players(&config);
        assert_eq!(usize::from(config.n_players), players.len());
    }

    #[test]
    fn create_the_right_number_of_players() {
        for n_players in 2..=MAX_N_PLAYERS {
            let args = [
                String::from("target/debug/tarot-cli"),
                String::from(n_players.to_string()),
            ];
            let config = Config::build(&args).unwrap();
            let players = create_players(&config);
            assert_eq!(usize::from(config.n_players), players.len());
        }
    }

    #[test]
    #[should_panic]

    const NUMBER_CARDS: u8 = 78;
    const MIN_NUMBER_CARDS_SPLIT: u8 = 3;
    const MAX_NUMBER_CARDS_SPLIT: u8 = NUMBER_CARDS - MIN_NUMBER_CARDS_SPLIT;
    const DEAL_SIZE_PLAYERS: usize = 3;
    const DEAL_SIZE_KITTY: usize = 1;

    #[derive(Debug)]
    pub struct Config {
        n_players: u8,
    }
    fn create_with_too_many_players_panics() {
        let args = [
            String::from("target/debug/tarot-cli"),
            String::from((MAX_N_PLAYERS + 1).to_string()),
        ];
        let config = Config::build(&args).unwrap();
        create_players(&config);
    }

    #[test]
    fn choose_only_one_first_dealer() {
        let args = [String::from("target/debug/tarot-cli")];
        let config = Config::build(&args).unwrap();
        let players = create_players(&config);
        let n_dealer = players.iter().filter(|p| p.is_dealer).count();
        assert_eq!(n_dealer, 1);
    }

    #[test]
    fn update_dealer_is_next_player() {
        let mut players = Vec::from([
            Player {
                name: String::from("Player 1"),
                score: 0,
                is_dealer: false,
                is_bot: false,
                cards: Vec::new(),
                played_cards: Vec::new(),
                picked_up_cards: Vec::new(),
            },
            Player {
                name: String::from("Player 2"),
                score: 0,
                is_dealer: true,
                is_bot: true,
                cards: Vec::new(),
                played_cards: Vec::new(),
                picked_up_cards: Vec::new(),
            },
            Player {
                name: String::from("Player 3"),
                score: 0,
                is_dealer: false,
                is_bot: true,
                cards: Vec::new(),
                played_cards: Vec::new(),
                picked_up_cards: Vec::new(),
            },
            Player {
                name: String::from("Player 4"),
                score: 0,
                is_dealer: false,
                is_bot: true,
                cards: Vec::new(),
                played_cards: Vec::new(),
                picked_up_cards: Vec::new(),
            },
        ]);
        assert_eq!(players[1].is_dealer, true);
        update_dealer(&mut players);
        assert_eq!(players[2].is_dealer, true);
        update_dealer(&mut players);
        assert_eq!(players[3].is_dealer, true);
        update_dealer(&mut players);
        assert_eq!(players[0].is_dealer, true);
    }
}

#[cfg(test)]
mod scoring {
    // use super::super::*;

    // #[test]
}

#[cfg(test)]
mod bid {
    use super::super::*;

    #[test]
    fn petite_is_higher_than_passe() {
        let previous_bid = Some(Bid::Passe);
        let bid = Bid::Petite;
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);
    }

    #[test]
    fn garde_is_higher_than_petite_and_passe() {
        let mut previous_bid = Some(Bid::Passe);
        let bid = Bid::Garde;
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::Petite);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);
    }

    #[test]
    fn garde_sans_is_higher_than_garde_and_petite_and_passe() {
        let mut previous_bid = Some(Bid::Passe);
        let bid = Bid::GardeSans;
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::Petite);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::Garde);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);
    }

    #[test]
    fn garde_contre_is_higher_than_garde_sans_and_garde_and_petite_and_passe() {
        let mut previous_bid = Some(Bid::Passe);
        let bid = Bid::GardeContre;
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::Petite);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::Garde);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);

        previous_bid = Some(Bid::GardeSans);
        assert_eq!(is_higher_bid(&bid, previous_bid.as_ref()), true);
    }
}
