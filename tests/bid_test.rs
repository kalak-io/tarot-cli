#[cfg(test)]
mod bid {
    use tarot_cli::common::bid::{compare_bids, Bids};

    #[test]
    fn petite_compare_than_passe() {
        let previous_bid = Bids::Passe;
        let bid = Bids::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_compare_than_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_sans_compare_than_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::GardeSans;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_contre_compare_than_garde_sans_and_garde_and_petite_and_passe() {
        let mut previous_bid = Bids::Passe;
        let bid = Bids::GardeContre;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bids::GardeSans;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }
}
