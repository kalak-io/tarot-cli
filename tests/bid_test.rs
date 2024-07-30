#[cfg(test)]
mod bid {
    use tarot_cli::common::bid::{compare_bids, Bid};

    #[test]
    fn petite_compare_than_passe() {
        let previous_bid = Bid::Passe;
        let bid = Bid::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_compare_than_petite_and_passe() {
        let mut previous_bid = Bid::Passe;
        let bid = Bid::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_sans_compare_than_garde_and_petite_and_passe() {
        let mut previous_bid = Bid::Passe;
        let bid = Bid::GardeSans;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }

    #[test]
    fn garde_contre_compare_than_garde_sans_and_garde_and_petite_and_passe() {
        let mut previous_bid = Bid::Passe;
        let bid = Bid::GardeContre;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::Petite;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::Garde;
        assert_eq!(compare_bids(&bid, &previous_bid), true);

        previous_bid = Bid::GardeSans;
        assert_eq!(compare_bids(&bid, &previous_bid), true);
    }
}
