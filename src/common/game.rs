use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::{
    card::{Card, CardSuits},
    player::{Player, PlayerActions, PlayerKind},
    utils::{get_next_index, random_int_in_range, reorder},
};

const NUMBER_CARDS_BY_SUIT: usize = 14;
const NUMBER_TRUMP_CARDS: usize = 22;

const TOTAL_CARDS: usize = 78;
const MIN_NUMBER_CARDS_SPLIT: usize = 3;
const MAX_NUMBER_CARDS_SPLIT: usize = TOTAL_CARDS - MIN_NUMBER_CARDS_SPLIT;

pub enum ReorderBy {
    Dealer,
}

pub trait GameActions {
    fn update_dealer(&mut self);
    fn split_deck(&mut self);
    fn collect_deck(&mut self, players: &[Player], kitty_cards: &[Card]);
    fn reorder_players(&mut self, by: ReorderBy);
    fn update_scores(&mut self, players: &[Player]);
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
}
impl Default for Game {
    fn default() -> Self {
        Game::new(4)
    }
}
impl Game {
    pub fn new(n_players: u8) -> Self {
        Game {
            players: create_players(n_players),
            deck: create_deck(),
        }
    }
}
impl GameActions for Game {
    fn split_deck(&mut self) {
        // Each part of the cut keeps more than 3 cards
        let split_index = random_int_in_range(MIN_NUMBER_CARDS_SPLIT + 1, MAX_NUMBER_CARDS_SPLIT);
        let mut new_deck = Vec::new();
        new_deck.extend_from_slice(&self.deck[split_index..]);
        new_deck.extend_from_slice(&self.deck[..split_index]);
        self.deck = new_deck;
    }

    fn collect_deck(&mut self, players: &[Player], kitty_cards: &[Card]) {
        let mut deck = Vec::new();
        for player in players {
            match player.hand.cards.is_empty() {
                true => deck.extend(player.hand.won_cards.clone()),
                false => deck.extend(player.hand.cards.clone()),
            }
        }
        deck.extend_from_slice(kitty_cards);
        self.deck = deck;
    }
    fn update_dealer(&mut self) {
        let index = find_dealer(&self.players);
        self.players[index].toggle_role();

        let next_index = get_next_index(&self.players, index);
        self.players[next_index].toggle_role();

        println!("The dealer is {}", self.players[next_index].name);
    }
    fn reorder_players(&mut self, by: ReorderBy) {
        let start_index = match by {
            ReorderBy::Dealer => {
                let dealer_index = find_dealer(&self.players);
                get_next_index(&self.players, dealer_index)
            }
        };
        let new_players = reorder(&self.players, start_index);
        self.players.clear();
        self.players.extend_from_slice(&new_players);
    }
    // Copy each player's running total back from the deal's copy of the players
    fn update_scores(&mut self, players: &[Player]) {
        for player in &mut self.players {
            if let Some(deal_player) = players.iter().find(|p| p.id == player.id) {
                player.update_score(deal_player.score() - player.score());
            }
        }
    }
}

// PLAYERS
// TODO: create a submodule game/players
fn generate_players(n_players: u8) -> Vec<Player> {
    let mut players = Vec::new();
    players.push(Player::new(
        "Player 1".to_string(),
        1,
        Some(PlayerKind::Human),
    ));
    for i in 2..=n_players {
        let player = Player::new(format!("Player {i}"), i, Some(PlayerKind::default()));
        players.push(player);
    }
    players
}

fn set_first_dealer(players: &mut [Player]) {
    let index = random_int_in_range(0, players.len());
    players[index].toggle_role();
}

fn create_players(n_players: u8) -> Vec<Player> {
    let mut players = generate_players(n_players);
    set_first_dealer(&mut players);
    players
}

fn generate_card(n_cards: usize, suit: CardSuits) -> Vec<Card> {
    let mut cards = Vec::new();
    for rank in 1..=n_cards {
        let card = Card::new(rank as u8, suit);
        cards.push(card);
    }
    cards.to_vec()
}

fn generate_suits(deck: &mut Vec<Card>) {
    for suit in CardSuits::AVAILABLE_SUITS.into_iter() {
        let n_cards = match suit {
            CardSuits::Trumps => NUMBER_TRUMP_CARDS,
            _ => NUMBER_CARDS_BY_SUIT,
        };
        deck.extend(generate_card(n_cards, suit));
    }
}

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    generate_suits(&mut deck);
    deck.shuffle(&mut thread_rng());
    deck.to_vec()
}

pub fn find_dealer(players: &[Player]) -> usize {
    players
        .iter()
        .position(|player| player.is_dealer())
        .unwrap()
}
