use crate::common::utils::{display, select};

use super::card::{Card, CardActions, CardSuits, CardSuitsGetters};

pub trait TrickActions {
    fn get_best_played_card_index(&self) -> Option<usize>;
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
    fn get_best_played_card_index(&self) -> Option<usize> {
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
    // TODO: implement way to check the trick
    // need to play the played_suit
    // need to play a superior trump if the played_suit is trumps or if the played_suit is not trump but you need to cut and another trump is played

    let played_suit = trick.played_suit();
    match (played_suit, player_selected_card.suit.name) {
        (Some(played_suit), CardSuits::Trumps) => {}
        (Some(played_suit), _) => {}
        (None, _) => true,
    }
    if played_suit != Some(player_selected_card.suit.name) {}
    let player_has_trumps = player_cards.iter().any(|card| card.suit.name.is_trump());
    let player_has_played_suit = player_cards
        .iter()
        .any(|card| card.suit.name == played_suit.unwrap());
    let selected_card_suit_is_same_than_played_suit =
        player_selected_card.suit.name == played_suit.unwrap();

    false
}

// filtrer les cartes du joueur selon les conditions suivantes:
// si le joueur a la couleur demandée et qu'il ne s'agit pas des atouts, il peut jouer la carte de son choix dans la couleur demandée
// si le joueur a la couleur demandée et qu'il s'agit des atouts alors il doit monter sur l'atout précédemment jouer
// si le joueur a la couleur demandée et qu'il s'agit des atouts et qu'il ne peut pas monter, il doit jouer un atout plus petit
// si le joueur n'a pas la couleur demandée et qu'il a des atouts, il doit jouer un attout
// si le joueur n'a pas la couleur demandée et qu'il n'a pas d'atout, il peut se défaussser
pub fn allowed_cards_to_play(trick: &Trick, player_cards: &[Card]) -> Vec<Card> {
    match trick.played_suit() {
        None => player_cards.to_vec(),
        Some(played_suit) => player_cards
            .iter()
            .filter(|card| card.suit.name == played_suit)
            .copied()
            .collect(),
    }
}
