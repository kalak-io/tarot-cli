#[cfg(test)]
mod player {
    use rstest::rstest;
    use tarot_cli::common::{
        card::{Card, CardSuits},
        player::{callable_cards, Player, PlayerActions, PlayerKind},
    };

    const SUITS: [CardSuits; 4] = [
        CardSuits::Clubs,
        CardSuits::Diamonds,
        CardSuits::Hearts,
        CardSuits::Spades,
    ];

    fn all_of_rank(rank: u8) -> Vec<Card> {
        SUITS.map(|suit| Card::new(rank, suit)).to_vec()
    }

    // "Le jeu à 5 joueurs": a King, then a Queen, Knight or Jack when all four of the rank above are held
    #[rstest]
    fn callable_cards_move_down_a_rank_when_all_four_are_held(
        #[values(
            (vec![], 14),
            (vec![Card::new(14, CardSuits::Clubs)], 14),
            (all_of_rank(14), 13),
            ([all_of_rank(14), all_of_rank(13)].concat(), 12),
            ([all_of_rank(14), all_of_rank(13), all_of_rank(12)].concat(), 11),
        )]
        case: (Vec<Card>, u8),
    ) {
        let (hand, expected_rank) = case;
        assert_eq!(callable_cards(&hand), all_of_rank(expected_rank));
    }

    #[rstest]
    fn bot_calls_a_card_it_does_not_hold(
        #[values(
            (vec![], Card::new(14, CardSuits::Clubs)),
            (vec![Card::new(14, CardSuits::Clubs)], Card::new(14, CardSuits::Diamonds)),
            (all_of_rank(14), Card::new(13, CardSuits::Clubs)),
            (
                [all_of_rank(14), vec![Card::new(13, CardSuits::Clubs), Card::new(13, CardSuits::Diamonds)]].concat(),
                Card::new(13, CardSuits::Hearts)
            ),
        )]
        case: (Vec<Card>, Card),
    ) {
        let (hand, expected) = case;
        let mut player = Player::new("Bot".to_string(), 1, Some(PlayerKind::Bot));
        player.hand.cards = hand;
        assert_eq!(player.call_king(), expected);
    }
}
