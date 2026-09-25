use std::fmt::Display;

use crate::common::{card::CardSuits, utils::select_card};

use super::{
    bid::{Bid, Bids},
    card::{Card, JACK_RANK, KING_RANK, KNIGHT_RANK, QUEEN_RANK},
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
    fn bid(&self, bid: &mut Bid, n_players: usize) -> Bids;
    fn call_king(&mut self) -> Card;
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card>;
    fn declare_poignee(&mut self, n_players: usize);
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
    fn bid(&self, bid: &mut Bid, n_players: usize) -> Bids {
        match self.kind {
            PlayerKind::Human => bid.human_choose(&self.hand.cards),
            PlayerKind::Bot => bid.bot_choose(&self.hand.cards, n_players),
        }
    }
    fn call_king(&mut self) -> Card {
        let callable = callable_cards(&self.hand.cards);
        match self.kind {
            PlayerKind::Human => human_call_king(&self.hand.cards, &callable),
            PlayerKind::Bot => bot_call_king(&self.hand.cards, &callable),
        }
    }
    fn compose_kitty(&mut self, kitty: &mut Kitty) -> Vec<Card> {
        self.hand.cards.extend(kitty.cards.clone());
        self.hand.sort_by_suit();

        match self.kind {
            PlayerKind::Human => kitty.human_compose(&mut self.hand.cards),
            PlayerKind::Bot => kitty.bot_compose(&mut self.hand.cards),
        }
    }
    fn declare_poignee(&mut self, n_players: usize) {
        match self.kind {
            PlayerKind::Human => self.hand.human_declare_poignee(n_players),
            PlayerKind::Bot => self.hand.bot_declare_poignee(n_players),
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

// "Le jeu à 5 joueurs": the taker calls a King. Holding all 4 Kings, they call a Queen,
// then a Knight, then a Jack. Calling a card from their own hand means playing alone.
pub fn callable_cards(cards: &[Card]) -> Vec<Card> {
    let suits = [
        CardSuits::Clubs,
        CardSuits::Diamonds,
        CardSuits::Hearts,
        CardSuits::Spades,
    ];
    let by_rank = |rank: u8| -> Vec<Card> { suits.map(|suit| Card::new(rank, suit)).to_vec() };
    [KING_RANK, QUEEN_RANK, KNIGHT_RANK]
        .into_iter()
        .map(by_rank)
        .find(|candidates| !candidates.iter().all(|card| cards.contains(card)))
        .unwrap_or_else(|| by_rank(JACK_RANK))
}

fn bot_call_king(cards: &[Card], callable: &[Card]) -> Card {
    callable
        .iter()
        .find(|card| !cards.contains(card))
        .copied()
        .unwrap_or(callable[0])
}

fn human_call_king(cards: &[Card], callable: &[Card]) -> Card {
    println!("\nYour cards:");
    display_cards(cards);
    select_card(Some("Which card do you call?"), Some(callable.to_vec())).unwrap()
}
