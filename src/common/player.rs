use std::fmt::Display;

use crate::common::{card::CardSuits, utils::select};

use super::{
    bid::{Bid, Bids},
    card::{Card, KING_RANK},
    hand::{Hand, HandActions},
    kitty::{Kitty, KittyActions},
    trick::{Trick, TrickActions},
    utils::display_cards,
};

#[derive(Debug, Default, Copy, Clone)]
pub enum PlayerKind {
    Human,
    #[default]
    Bot,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
enum PlayerRole {
    Dealer,
    #[default]
    Receiver,
}

pub trait PlayerActions {
    fn bid(&self, bid: &mut Bid) -> Bids;
    fn call_king(&mut self) -> Card;
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card>;
    fn declare_poignee(&mut self);
    fn declare_chelem(&mut self);
    fn play(&mut self, trick: &mut Trick);
    fn update_score(&mut self, score: f64);
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub name: String,
    score: f64,
    kind: PlayerKind,
    role: PlayerRole,
    pub cards: Vec<Card>,
    pub picked_up_cards: Vec<Card>,
    pub hand: Hand,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}
impl Player {
    pub fn new(name: String, id: u8, kind: Option<PlayerKind>) -> Self {
        Player {
            id,
            name,
            kind: kind.unwrap_or_default(),
            ..Default::default()
        }
    }
    pub fn score(&self) -> f64 {
        self.score
    }
    pub fn is_dealer(&self) -> bool {
        self.role == PlayerRole::Dealer
    }
    pub fn toggle_role(&mut self) {
        self.role = match self.role {
            PlayerRole::Dealer => PlayerRole::Receiver,
            PlayerRole::Receiver => PlayerRole::Dealer,
        }
    }
}
impl PlayerActions for Player {
    fn bid(&self, bid: &mut Bid) -> Bids {
        match self.kind {
            PlayerKind::Human => bid.human_choose(&self.hand.cards),
            PlayerKind::Bot => bid.bot_choose(&self.hand.cards),
        }
    }
    fn call_king(&mut self) -> Card {
        let kings: [Card; 4] = [
            Card::new(KING_RANK, CardSuits::Clubs),
            Card::new(KING_RANK, CardSuits::Diamonds),
            Card::new(KING_RANK, CardSuits::Hearts),
            Card::new(KING_RANK, CardSuits::Spades),
        ];
        match self.kind {
            PlayerKind::Human => human_call_king(&self.hand.cards, &kings),
            PlayerKind::Bot => bot_call_king(&self.hand.cards, &kings),
        }
    }
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card> {
        self.hand.cards.extend(kitty.cards.clone());
        self.hand
            .cards
            .sort_unstable_by_key(|card| (card.suit.initial, card.rank));

        match self.kind {
            PlayerKind::Human => kitty.human_compose(&mut self.hand.cards),
            PlayerKind::Bot => kitty.bot_compose(&mut self.hand.cards),
        }
    }
    fn declare_poignee(&mut self) {
        match self.kind {
            PlayerKind::Human => self.hand.human_declare_poignee(),
            PlayerKind::Bot => self.hand.bot_declare_poignee(),
        }
    }
    fn declare_chelem(&mut self) {
        match self.kind {
            PlayerKind::Human => self.hand.human_declare_chelem(),
            PlayerKind::Bot => self.hand.bot_declare_chelem(),
        }
    }
    fn play(&mut self, trick: &mut Trick) {
        match self.kind {
            PlayerKind::Human => trick.human_play(&mut self.hand.cards),
            PlayerKind::Bot => trick.bot_play(&mut self.hand.cards),
        }
    }
    fn update_score(&mut self, score: f64) {
        self.score += score;
    }
}

fn bot_call_king(cards: &[Card], kings: &[Card]) -> Card {
    kings
        .iter()
        .find(|king| !cards.contains(king))
        .copied()
        .unwrap_or(kings[0])
}

fn human_call_king(cards: &[Card], kings: &[Card]) -> Card {
    println!("\nYour cards:");
    display_cards(cards);
    select(Some("Which king do you call?"), Some(kings.to_vec())).unwrap()
}
