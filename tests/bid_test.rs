#[cfg(test)]
mod bid {
    use rstest::rstest;
    use tarot_cli::common::bid::{compare_bids, Bid, Bids};

    #[test]
    fn petite_compare_than_passe() {
        let previous_bid = Bids::Passe;
        let bid = Bids::Petite;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_compare_than_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::Garde;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Petite;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_sans_compare_than_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::GardeSans;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Petite;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Garde;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[test]
    fn garde_contre_compare_than_garde_sans_and_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::GardeContre;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Petite;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::Garde;
        assert!(compare_bids(&bid, &previous_bid));

        previous_bid = Bids::GardeSans;
        assert!(compare_bids(&bid, &previous_bid));
    }

    #[rstest]
    fn computes_available_bids_by_bid(
        #[values((Bids::Petite, 4), (Bids::Garde, 3), (Bids::GardeSans, 2), (Bids::GardeContre, 1), (Bids::Passe, 5))]
        case: (Bids, usize),
    ) {
        let (bid_name, expected_availables_bids_len) = case;
        let bid = Bid::new(bid_name);
        assert_eq!(bid.get_available_bids().len(), expected_availables_bids_len);
    }
}
