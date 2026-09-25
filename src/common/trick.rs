use crate::common::utils::{display_cards, select_card};

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
    // Set on the first trick of a 5-player deal only
    pub called_card: Option<Card>,
}

impl TrickActions for Trick {
    fn get_best_played_card_index(&self, played_suit: Option<CardSuits>) -> Option<usize> {
        let played_suit = played_suit.or_else(|| self.played_suit())?;
        // The Fool never wins a trick
        self.played_cards
            .iter()
            .enumerate()
            .filter(|(_, card)| !card.is_fool())
            .reduce(|best, current| {
                if current.1.is_superior_than(best.1, Some(played_suit)) {
                    current
                } else {
                    best
                }
            })
            .map(|(index, _)| index)
    }

    fn human_play(&mut self, cards: &mut Vec<Card>) {
        println!("\nYour cards:");
        display_cards(cards);
        let allowed = allowed_cards_to_play(self, cards);
        let card = select_card(Some("Which card do you play?"), Some(allowed)).unwrap();

        let index = cards
            .iter()
            .position(|c| c.suit.name == card.suit.name && c.rank == card.rank)
            .unwrap();
        cards.remove(index);

        self.played_cards.push(card);
    }

    fn bot_play(&mut self, cards: &mut Vec<Card>) {
        let (fool, allowed): (Vec<Card>, Vec<Card>) = allowed_cards_to_play(self, cards)
            .into_iter()
            .partition(|c| c.is_fool());
        // Play the Fool before the last trick, where it would go to the other side
        let card = match fool.first() {
            Some(fool) if allowed.is_empty() || cards.len() <= 2 => *fool,
            _ => choose_best_card(self, &allowed),
        };
        let pos = cards
            .iter()
            .position(|c| c.suit.name == card.suit.name && c.rank == card.rank)
            .unwrap();
        cards.remove(pos);
        self.played_cards.push(card);
    }
}

impl TrickGetters for Trick {
    // When the Fool leads, the next card sets the suit
    fn played_suit(&self) -> Option<CardSuits> {
        self.played_cards
            .iter()
            .find(|card| !card.is_fool())
            .map(|card| card.suit.name)
    }
    fn has_petit_au_bout(&self) -> bool {
        self.played_cards.contains(&Card::new(1, CardSuits::Trumps))
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

// When a player must play a trump, they must beat the highest trump already played if they can
fn trumps_to_play(trick: &Trick, trumps: Vec<Card>) -> Vec<Card> {
    let highest_played_trump = trick
        .played_cards
        .iter()
        .filter(|card| card.suit.is_trump() && !card.is_fool())
        .map(|card| card.rank)
        .max();
    let higher_trumps: Vec<Card> = trumps
        .iter()
        .filter(|card| highest_played_trump.is_none_or(|rank| card.rank > rank))
        .copied()
        .collect();
    if higher_trumps.is_empty() {
        trumps
    } else {
        higher_trumps
    }
}

pub fn allowed_cards_to_play(trick: &Trick, player_cards: &[Card]) -> Vec<Card> {
    // The Fool can be played at any time and does not count as a trump here
    let (fool, cards): (Vec<Card>, Vec<Card>) = player_cards
        .iter()
        .copied()
        .partition(|card| card.is_fool());

    let mut allowed_cards = match trick.played_suit() {
        // "Le jeu à 5 joueurs": the first lead is not in the called card's suit,
        // unless it is the called card itself
        None => match trick.called_card {
            Some(called) => {
                let outside_called_suit: Vec<Card> = cards
                    .iter()
                    .filter(|card| card.suit.name != called.suit.name || **card == called)
                    .copied()
                    .collect();
                if outside_called_suit.is_empty() {
                    cards
                } else {
                    outside_called_suit
                }
            }
            None => cards,
        },
        Some(played_suit) => {
            let suit_cards: Vec<Card> = cards
                .iter()
                .filter(|card| card.suit.name == played_suit)
                .copied()
                .collect();
            let trumps: Vec<Card> = cards
                .iter()
                .filter(|card| card.suit.is_trump())
                .copied()
                .collect();

            if played_suit != CardSuits::Trumps && !suit_cards.is_empty() {
                // Follow the led suit, with no need to beat the cards played
                suit_cards
            } else if !trumps.is_empty() {
                // Trumps led, or no card of the led suit: play a trump, over-trumping if possible
                trumps_to_play(trick, trumps)
            } else {
                // No card of the led suit and no trump: discard any card
                cards
            }
        }
    };
    allowed_cards.extend(fool);
    allowed_cards
}
