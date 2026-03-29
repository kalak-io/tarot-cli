#[cfg(test)]
mod bid {
    use rstest::rstest;
    use tarot_cli::common::bid::{compare_bids, taker_evaluation, Bid, Bids};
    use tarot_cli::common::card::{Card, CardSuits};

    #[test]
    fn petite_compare_than_passe() {
        let previous_bid = Bids::Pass;
        let bid = Bids::Take;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_compare_than_petite_and_passe() {
        let mut previous_bid = Bids::Pass;
        let bid = Bids::Guard;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Take;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_sans_compare_than_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Pass;
        let bid = Bids::GuardWithout;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Take;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Guard;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_contre_compare_than_garde_sans_and_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Pass;
        let bid = Bids::GuardAgainst;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Take;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Guard;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::GuardWithout;
        assert!(compare_bids(&bid, &previous_bid));
    }

    // taker_evaluation formula: n_oudlers * (compute_points(cards) % 5.0)
    // 0..2 → Pass, 2..4 → Take, 4..6 → Guard, 6..8 → GuardWithout, 8+ → GuardAgainst
    #[rstest]
    fn taker_evaluation_returns_correct_bid(
        #[values(
            // 0 oudlers → evaluation = 0 → Pass
            (vec![], Bids::Pass),
            // 1 oudler (Fool=4.5) + Queen (3.5) = 8.0 pts; 8.0 % 5 = 3.0; eval = 1 × 3.0 = 3.0 → Take
            (vec![Card::new(22, CardSuits::Trumps), Card::new(13, CardSuits::Hearts)], Bids::Take),
            // 1 oudler (Little=4.5) + King (4.5) = 9.0 pts; 9.0 % 5 = 4.0; eval = 1 × 4.0 = 4.0 → Guard
            (vec![Card::new(1, CardSuits::Trumps), Card::new(14, CardSuits::Hearts)], Bids::Guard),
            // 2 oudlers (Fool+Big=9.0) + King (4.5) = 13.5 pts; 13.5 % 5 = 3.5; eval = 2 × 3.5 = 7.0 → GuardWithout
            (vec![Card::new(22, CardSuits::Trumps), Card::new(21, CardSuits::Trumps), Card::new(14, CardSuits::Hearts)], Bids::GuardWithout),
            // 3 oudlers (Fool+Little+Big=13.5) + King (4.5) = 18.0 pts; 18.0 % 5 = 3.0; eval = 3 × 3.0 = 9.0 → GuardAgainst
            (vec![Card::new(22, CardSuits::Trumps), Card::new(1, CardSuits::Trumps), Card::new(21, CardSuits::Trumps), Card::new(14, CardSuits::Hearts)], Bids::GuardAgainst),
        )]
        case: (Vec<Card>, Bids),
    ) {
        let (cards, expected) = case;
        assert_eq!(taker_evaluation(&cards), expected);
    }

    #[rstest]
    fn computes_available_bids_by_bid(
        #[values((Bids::Take, 4), (Bids::Guard, 3), (Bids::GuardWithout, 2), (Bids::GuardAgainst, 1), (Bids::Pass, 5))]
        case: (Bids, usize),
    ) {
        let (bid_name, expected_availables_bids_len) = case;
        let bid = Bid::new(bid_name);
        assert_eq!(bid.get_available_bids().len(), expected_availables_bids_len);
    }
}
