#[cfg(test)]
mod utils {
    use tarot_cli::common::{
        card::{Card, CardSuits},
        utils::{card_box_lines, card_rank_label, compare, get_next_index, reorder, subtract},
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
        let mut hand = vec![
            Card::new(2, CardSuits::Clubs),
            Card::new(5, CardSuits::Hearts),
        ];
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

    #[test]
    fn card_rank_label_centers_single_digit() {
        assert_eq!(card_rank_label(&Card::new(7, CardSuits::Hearts)), " 7 ");
        assert_eq!(card_rank_label(&Card::new(1, CardSuits::Hearts)), " 1 ");
    }

    #[test]
    fn card_rank_label_left_aligns_double_digit() {
        assert_eq!(card_rank_label(&Card::new(10, CardSuits::Hearts)), "10 ");
        assert_eq!(card_rank_label(&Card::new(16, CardSuits::Trumps)), "16 ");
        assert_eq!(card_rank_label(&Card::new(21, CardSuits::Trumps)), "21 ");
    }

    #[test]
    fn card_rank_label_abbreviates_face_cards() {
        assert_eq!(card_rank_label(&Card::new(11, CardSuits::Hearts)), "Jck");
        assert_eq!(card_rank_label(&Card::new(12, CardSuits::Hearts)), "Knt");
        assert_eq!(card_rank_label(&Card::new(13, CardSuits::Hearts)), "Que");
        assert_eq!(card_rank_label(&Card::new(14, CardSuits::Hearts)), "Kng");
        assert_eq!(card_rank_label(&Card::new(22, CardSuits::Trumps)), "Foo");
    }

    #[test]
    fn card_box_lines_regular_card_uses_single_borders() {
        let cards = vec![Card::new(10, CardSuits::Spades)];
        let lines = card_box_lines(&cards);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "┌───┐");
        assert_eq!(lines[1], "│10 │");
        assert_eq!(lines[2], "│ ♠ │");
        assert_eq!(lines[3], "└───┘");
    }

    #[test]
    fn card_box_lines_oudler_uses_double_borders() {
        // Little (rank 1 trump) is an oudler
        let cards = vec![Card::new(1, CardSuits::Trumps)];
        let lines = card_box_lines(&cards);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "╔═══╗");
        assert_eq!(lines[1], "│ 1 │");
        assert_eq!(lines[2], "│ ★ │");
        assert_eq!(lines[3], "╚═══╝");
    }

    #[test]
    fn card_box_lines_fool_uses_double_borders() {
        let cards = vec![Card::new(22, CardSuits::Trumps)];
        let lines = card_box_lines(&cards);
        assert_eq!(lines[0], "╔═══╗");
        assert_eq!(lines[1], "│Foo│");
        assert_eq!(lines[2], "│ ★ │");
        assert_eq!(lines[3], "╚═══╝");
    }

    #[test]
    fn card_box_lines_multiple_cards_are_space_separated() {
        let cards = vec![
            Card::new(10, CardSuits::Spades),
            Card::new(5, CardSuits::Hearts),
        ];
        let lines = card_box_lines(&cards);
        assert_eq!(lines[0], "┌───┐ ┌───┐");
        assert_eq!(lines[1], "│10 │ │ 5 │");
        assert_eq!(lines[2], "│ ♠ │ │ ♥ │");
        assert_eq!(lines[3], "└───┘ └───┘");
    }

    #[test]
    fn card_box_lines_wraps_at_nine_cards() {
        // 10 cards → row of 9 + row of 1 → 8 lines total
        let cards: Vec<Card> = (1u8..=10)
            .map(|r| Card::new(r, CardSuits::Hearts))
            .collect();
        let lines = card_box_lines(&cards);
        assert_eq!(lines.len(), 8);
    }

    #[test]
    fn card_rank_label_is_always_three_chars() {
        for (rank, suit) in [
            (1u8, CardSuits::Hearts),
            (5, CardSuits::Hearts),
            (9, CardSuits::Hearts),
            (10, CardSuits::Hearts),
            (16, CardSuits::Trumps),
            (21, CardSuits::Trumps),
            (11, CardSuits::Clubs),
            (14, CardSuits::Spades),
            (22, CardSuits::Trumps),
        ] {
            let label = card_rank_label(&Card::new(rank, suit));
            assert_eq!(
                label.chars().count(),
                3,
                "rank {} label '{}' should be 3 chars",
                rank,
                label
            );
        }
    }
}
