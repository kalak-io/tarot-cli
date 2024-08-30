#[cfg(test)]
mod bid {
    use tarot_cli::common::bid::{compare_bids, Bid, Bids};

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

    #[test]
    fn available_bids_with_current_passe() {
        let bid = Bid::new(Bids::Passe);
        assert_eq!(bid.get_available_bids().len(), 5);
    }

    #[test]
    fn available_bids_with_current_petite() {
        let bid = Bid::new(Bids::Petite);
        assert_eq!(bid.get_available_bids().len(), 4);
    }

    #[test]
    fn available_bids_with_current_garde() {
        let bid = Bid::new(Bids::Garde);
        assert_eq!(bid.get_available_bids().len(), 3);
    }

    #[test]
    fn available_bids_with_current_garde_sans() {
        let bid = Bid::new(Bids::GardeSans);
        assert_eq!(bid.get_available_bids().len(), 2);
    }

    #[test]
    fn available_bids_with_current_garde_contre() {
        let bid = Bid::new(Bids::GardeContre);
        assert_eq!(bid.get_available_bids().len(), 1);
    }
}
