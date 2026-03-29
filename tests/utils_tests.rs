#[cfg(test)]
mod utils {
    use tarot_cli::common::{
        card::{Card, CardSuits},
        utils::{compare, get_next_index, reorder, subtract},
    };

    // get_next_index

    #[test]
    fn get_next_index_advances_within_bounds() {
        let v = [1, 2, 3, 4];
        assert_eq!(get_next_index(&v, 0), 1);
        assert_eq!(get_next_index(&v, 2), 3);
    }

    #[test]
    fn get_next_index_wraps_around_at_end() {
        let v = [1, 2, 3];
        assert_eq!(get_next_index(&v, 2), 0);
    }

    #[test]
    fn get_next_index_single_element_returns_zero() {
        let v = [42];
        assert_eq!(get_next_index(&v, 0), 0);
    }

    // reorder

    #[test]
    fn reorder_from_zero_is_identity() {
        let v = [1, 2, 3, 4];
        assert_eq!(reorder(&v, 0), vec![1, 2, 3, 4]);
    }

    #[test]
    fn reorder_from_middle_rotates_correctly() {
        let v = [1, 2, 3, 4];
        assert_eq!(reorder(&v, 2), vec![3, 4, 1, 2]);
    }

    #[test]
    fn reorder_from_last_puts_last_element_first() {
        let v = [1, 2, 3, 4];
        assert_eq!(reorder(&v, 3), vec![4, 1, 2, 3]);
    }

    // compare

    #[test]
    fn compare_with_none_always_returns_true() {
        assert!(compare(&1usize, None, |a, b| a > b));
        assert!(compare(&0usize, None, |a, b| a > b));
    }

    #[test]
    fn compare_with_some_delegates_to_comparator() {
        assert!(compare(&3usize, Some(&2), |a, b| a > b));
        assert!(!compare(&1usize, Some(&2), |a, b| a > b));
        assert!(!compare(&2usize, Some(&2), |a, b| a > b));
    }

    // subtract

    #[test]
    fn subtract_removes_matching_cards() {
        let clubs2 = Card::new(2, CardSuits::Clubs);
        let hearts5 = Card::new(5, CardSuits::Hearts);
        let spades3 = Card::new(3, CardSuits::Spades);

        let mut hand = vec![clubs2, hearts5, spades3];
        subtract(&mut hand, &[hearts5]);
        assert_eq!(hand, vec![clubs2, spades3]);
    }

    #[test]
    fn subtract_with_empty_slice_leaves_hand_unchanged() {
        let mut hand = vec![Card::new(2, CardSuits::Clubs), Card::new(5, CardSuits::Hearts)];
        let original = hand.clone();
        subtract(&mut hand, &[]);
        assert_eq!(hand, original);
    }

    #[test]
    fn subtract_removes_multiple_cards() {
        let a = Card::new(2, CardSuits::Clubs);
        let b = Card::new(5, CardSuits::Hearts);
        let c = Card::new(3, CardSuits::Spades);

        let mut hand = vec![a, b, c];
        subtract(&mut hand, &[a, c]);
        assert_eq!(hand, vec![b]);
    }

    #[test]
    fn subtract_ignores_cards_not_in_hand() {
        let a = Card::new(2, CardSuits::Clubs);
        let b = Card::new(5, CardSuits::Hearts);
        let absent = Card::new(9, CardSuits::Diamonds);

        let mut hand = vec![a, b];
        subtract(&mut hand, &[absent]);
        assert_eq!(hand, vec![a, b]);
    }
}
