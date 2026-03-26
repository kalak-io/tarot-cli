use crate::common::utils::{display, select};

use super::{
    card::{Card, CardActions, CardGetters, CardSuits, CardSuitsGetters, KING_RANK},
    hand::Side,
};

pub trait TrickActions {
    fn get_best_played_card_index(&self, played_suit: Option<CardSuits>) -> Option<usize>;
    fn human_play(&mut self, cards: &mut Vec<Card>);
    fn bot_play(&mut self, cards: &mut Vec<Card>);
}

pub trait TrickGetters {
    fn played_suit(&self) -> Option<CardSuits>;
    fn has_petit_au_bout(&self) -> bool;
}

#[derive(Debug, Default)]
pub struct Trick {
    pub played_cards: Vec<Card>,
    pub winner_side: Side,
}

impl TrickActions for Trick {
    fn get_best_played_card_index(&self, played_suit: Option<CardSuits>) -> Option<usize> {
        if self.played_cards.is_empty() {
            return None;
        }
        let mut best_card_index = 0;
        let played_suit = played_suit.or_else(|| self.played_suit());

        for (index, card) in self.played_cards.iter().enumerate() {
            if card.is_superior_than(&self.played_cards[best_card_index], Some(played_suit?)) {
                best_card_index = index;
            }
        }

        Some(best_card_index)
    }

    fn human_play(&mut self, cards: &mut Vec<Card>) {
        println!("\nYour cards:");
        display(cards);
        let card = select(
            Some("Which card do you play?"),
            Some(self.played_cards.to_vec()),
        )
        .unwrap();

        if check_selected_card(self, cards, &card).is_err() {
            self.human_play(cards);
        }

        let index = cards
            .iter()
            .position(|c| c.suit.name == card.suit.name && c.rank == card.rank)
            .unwrap();
        cards.remove(index);

        self.played_cards.push(card);
    }

    fn bot_play(&mut self, cards: &mut Vec<Card>) {
        let allowed = allowed_cards_to_play(self, cards);
        let card = choose_best_card(self, &allowed);
        let pos = cards
            .iter()
            .position(|c| c.suit.name == card.suit.name && c.rank == card.rank)
            .unwrap();
        cards.remove(pos);
        self.played_cards.push(card);
    }
}

impl TrickGetters for Trick {
    fn played_suit(&self) -> Option<CardSuits> {
        if self.played_cards.is_empty() {
            return None;
        }
        Some(self.played_cards[0].suit.name)
    }
    fn has_petit_au_bout(&self) -> bool {
        self.played_cards.contains(&Card::new(1, CardSuits::Trumps))
    }
}

pub fn check_selected_card(
    trick: &Trick,
    player_cards: &[Card],
    player_selected_card: &Card,
) -> Result<bool, &'static str> {
    let allowed_cards = allowed_cards_to_play(trick, player_cards);
    if allowed_cards.contains(player_selected_card) {
        Ok(true)
    } else {
        Err("Selected card is not allowed to be played")
    }
}

fn is_winning_card(card: &Card, trick: &Trick) -> bool {
    let played_suit = trick.played_suit();
    match trick.get_best_played_card_index(played_suit) {
        None => true,
        Some(i) => card.is_superior_than(&trick.played_cards[i], played_suit),
    }
}

fn trick_point_value(trick: &Trick) -> f64 {
    trick.played_cards.iter().map(|c| c.score()).sum()
}

fn lead_card_priority(card: &Card) -> i32 {
    if card.is_oudler() {
        return 100;
    }
    if card.suit.is_trump() {
        // Lead low trumps before high ones, but prefer non-trumps first
        return 50 + card.rank as i32;
    }
    if card.rank == KING_RANK {
        return 40;
    }
    // Prefer cheap non-trump cards
    (card.score() * 10.0) as i32
}

fn choose_best_card(trick: &Trick, allowed: &[Card]) -> Card {
    if trick.played_cards.is_empty() {
        // Leading: play cheapest safe card (never waste Oudlers or Kings)
        *allowed
            .iter()
            .min_by_key(|c| lead_card_priority(c))
            .unwrap()
    } else {
        let trick_value = trick_point_value(trick);
        let winning: Vec<Card> = allowed
            .iter()
            .filter(|c| is_winning_card(c, trick))
            .cloned()
            .collect();

        if winning.is_empty() || trick_value < 1.5 {
            // Can't win, or trick not worth winning: discard cheapest card
            *allowed
                .iter()
                .min_by_key(|c| (c.score() * 10.0) as i32)
                .unwrap()
        } else {
            // Trick has value: win with cheapest winning card, sparing Oudlers if possible
            let non_oudler_wins: Vec<Card> =
                winning.iter().filter(|c| !c.is_oudler()).cloned().collect();
            let candidates = if non_oudler_wins.is_empty() {
                &winning
            } else {
                &non_oudler_wins
            };
            *candidates
                .iter()
                .min_by_key(|c| (c.score() * 10.0) as i32)
                .unwrap()
        }
    }
}

pub fn allowed_cards_to_play(trick: &Trick, player_cards: &[Card]) -> Vec<Card> {
    let mut allowed_cards = Vec::with_capacity(player_cards.len());
    let played_suit = trick.played_suit();

    match played_suit {
        None => allowed_cards.extend_from_slice(player_cards),
        Some(played_suit) => {
            let has_played_suit = player_cards
                .iter()
                .any(|card| card.suit.name == played_suit);
            let has_trumps = player_cards.iter().any(|card| card.suit.is_trump());

            if trick.played_suit() == Some(CardSuits::Trumps) {
                if has_trumps {
                    let best_played_trump_index = trick
                        .get_best_played_card_index(Some(CardSuits::Trumps))
                        .unwrap();
                    let best_played_trump = trick.played_cards[best_played_trump_index];

                    let superior_trumps: Vec<Card> = player_cards
                        .iter()
                        .filter(|card| {
                            card.suit.is_trump()
                                && card
                                    .is_superior_than(&best_played_trump, Some(CardSuits::Trumps))
                        })
                        .cloned()
                        .collect();

                    if superior_trumps.is_empty() {
                        // If no superior trumps are found, allow all trumps to be played
                        for card in player_cards {
                            if card.suit.is_trump() {
                                allowed_cards.push(*card);
                            }
                        }
                    } else {
                        allowed_cards.extend(superior_trumps);
                    }
                } else {
                    // If the player doesn't have trump cards, allow them to play any card
                    allowed_cards.extend_from_slice(player_cards);
                }
            } else if has_played_suit {
                // If the player has cards of the same suit as the first card played, only allow them to play those cards
                for card in player_cards {
                    if card.suit.name == played_suit {
                        allowed_cards.push(*card);
                    }
                }
            } else if has_trumps {
                // If the player has trump cards, but not cards of the same suit as the first card played, only allow them to play trump cards
                for card in player_cards {
                    if card.suit.is_trump() {
                        allowed_cards.push(*card);
                    }
                }
            } else {
                // If the player doesn't have cards of the same suit as the first card played, and doesn't have trump cards, allow them to play any card
                allowed_cards.extend_from_slice(player_cards);
            }
        }
    }
    allowed_cards
}
