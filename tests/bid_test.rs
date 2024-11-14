#[cfg(test)]
mod bid {
    use rstest::rstest;
    use tarot_cli::common::bid::{compare_bids, Bid, Bids};

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
