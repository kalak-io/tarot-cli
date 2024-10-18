use crate::common::utils::{display, select};

use super::card::{Card, CardActions, CardSuits, CardSuitsGetters};

pub trait TrickActions {
    fn get_best_played_card_index(&self, played_suit: Option<CardSuits>) -> Option<usize>;
    fn human_play(&mut self, cards: &mut Vec<Card>);
    fn bot_play(&mut self, cards: &mut Vec<Card>);
}

pub trait TrickGetters {
    fn played_suit(&self) -> Option<CardSuits>;
}

#[derive(Debug, Default)]
pub struct Trick {
    pub played_cards: Vec<Card>,
}

impl TrickActions for Trick {
    fn get_best_played_card_index(&self, played_suit: Option<CardSuits>) -> Option<usize> {
        if self.played_cards.is_empty() {
            return None;
        }
        let mut best_card_index = 0;
        let played_suit = self.played_suit();

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

        // TODO: the validation of choose of the human
        // A player must to play played_suit
        // if he has not card vith the same played_suit, the player must use a trump
        // if a trump is already played, the new trump must to be superior else play a trump
        // if there are no card with the same played_suit or no trump, the player can play any card

        let index = cards
            .iter()
            .position(|c| c.suit.name == card.suit.name && c.rank == card.rank)
            .unwrap();
        cards.remove(index);

        self.played_cards.push(card);
    }

    fn bot_play(&mut self, _cards: &mut Vec<Card>) {
        todo!("Implement bot play")
    }
}

impl TrickGetters for Trick {
    fn played_suit(&self) -> Option<CardSuits> {
        if self.played_cards.is_empty() {
            return None;
        }
        Some(self.played_cards[0].suit.name)
    }
}

pub fn check_selected_card(
    trick: &Trick,
    player_cards: &[Card],
    player_selected_card: &Card,
) -> bool {
    let allowed_cards = allowed_cards_to_play(trick, player_cards);
    allowed_cards.contains(player_selected_card)
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
    println!("Allowed cards to play: {:?}", allowed_cards);
    allowed_cards
}
// pub fn allowed_cards_to_play(trick: &Trick, player_cards: &[Card]) -> Vec<Card> {
//     let mut allowed_cards = Vec::with_capacity(player_cards.len());
//     let played_suit = trick.played_suit();

//     match played_suit {
//         None => allowed_cards.extend_from_slice(player_cards),
//         Some(played_suit) => {
//             let has_played_suit = player_cards.iter().any(|card| card.suit.name == played_suit);
//             let has_trumps = player_cards.iter().any(|card| card.suit.is_trump());

//             if trick.played_suit() == Some(CardSuits::Trumps) {
//                 let best_played_trump_index = trick.get_best_played_card_index(Some(CardSuits::Trumps)).unwrap();
//                 let best_played_trump = trick.played_cards[best_played_trump_index];
//                 for card in player_cards {
//                     if card.suit.name == played_suit || (has_trumps && card.is_superior_than(&best_played_trump, Some(CardSuits::Trumps))) {
//                         allowed_cards.push(*card);
//                     }
//                 }
//             } else if has_played_suit {
//                 for card in player_cards {
//                     if card.suit.name == played_suit {
//                         allowed_cards.push(*card);
//                     }
//                 }
//             } else if has_trumps {
//                 for card in player_cards {
//                     if card.suit.is_trump() {
//                         allowed_cards.push(*card);
//                     }
//                 }
//             } else {
//                 allowed_cards.extend_from_slice(player_cards);
//             }
//         }
//     }

//     allowed_cards
// }
