#[cfg(test)]
mod bid {
    use rstest::rstest;
    use tarot_cli::common::bid::{compare_bids, hand_strength, taker_evaluation, Bid, Bids};
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

    fn trumps(ranks: std::ops::RangeInclusive<u8>) -> Vec<Card> {
        ranks
            .map(|rank| Card::new(rank, CardSuits::Trumps))
            .collect()
    }

    fn kings(n: usize) -> Vec<Card> {
        [
            CardSuits::Clubs,
            CardSuits::Diamonds,
            CardSuits::Hearts,
            CardSuits::Spades,
        ]
        .into_iter()
        .take(n)
        .map(|suit| Card::new(14, suit))
        .collect()
    }

    // hand_strength: 21 = 10, Fool = 8, Little = 5, other trumps 2 (3 from the 16 up),
    // King 6, Queen 3, Knight 2, Jack 1, +5 per suit of 5 cards or more.
    // Pass < 36, Take < 42, Guard < 50, Guard Without < 57, then Guard Against.
    #[rstest]
    fn taker_evaluation_returns_correct_bid(
        #[values(
            (vec![], 0, Bids::Pass),
            // 3 low trumps (6) + 1 King (6) = 12
            ([trumps(2..=4), kings(1)].concat(), 12, Bids::Pass),
            // 21 + Fool (18) + 6 low trumps (12) + 1 King (6) = 36
            ([vec![Card::new(21, CardSuits::Trumps), Card::new(22, CardSuits::Trumps)], trumps(2..=7), kings(1)].concat(), 36, Bids::Take),
            // 3 oudlers (23) + 7 low trumps (14) + 1 King (6) = 43
            ([trumps(1..=8), vec![Card::new(21, CardSuits::Trumps), Card::new(22, CardSuits::Trumps)], kings(1)].concat(), 43, Bids::Guard),
            // 3 oudlers (23) + 7 low trumps (14) + 3 Kings (18) = 55
            ([trumps(1..=8), vec![Card::new(21, CardSuits::Trumps), Card::new(22, CardSuits::Trumps)], kings(3)].concat(), 55, Bids::GuardWithout),
            // 3 oudlers (23) + trumps 12 to 20 (8 + 15) + 4 Kings (24) = 70
            ([vec![Card::new(1, CardSuits::Trumps)], trumps(12..=22), kings(4)].concat(), 70, Bids::GuardAgainst),
            // A suit of 5 cards adds 5: 5 low hearts (0) + 5 = 5
            ((2..=6).map(|rank| Card::new(rank, CardSuits::Hearts)).collect(), 5, Bids::Pass),
        )]
        case: (Vec<Card>, u32, Bids),
    ) {
        let (cards, expected_strength, expected_bid) = case;
        assert_eq!(hand_strength(&cards), expected_strength);
        assert_eq!(taker_evaluation(&cards), expected_bid);
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

    #[test]
    fn record_pass_keeps_the_highest_bid() {
        let mut bid = Bid::new(Bids::Guard);
        assert_eq!(bid.record(Bids::Pass), Bids::Pass);
        assert_eq!(bid.current, Bids::Guard);
        assert!(!bid.get_available_bids().contains(&Bids::Take));
    }

    #[test]
    fn record_higher_bid_replaces_current_bid() {
        let mut bid = Bid::new(Bids::Take);
        assert_eq!(bid.record(Bids::GuardWithout), Bids::GuardWithout);
        assert_eq!(bid.current, Bids::GuardWithout);
    }
}
