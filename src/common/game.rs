use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::card::{Card, CardSuits};
use super::deal::Deal;
use super::player::Player;
use super::utils::{get_next_index, random_int_in_range, reorder};

const NUMBER_CARDS_BY_SUIT: usize = 14;
const NUMBER_TRUMP_CARDS: usize = 22;

const TOTAL_CARDS: usize = 78;
const MIN_NUMBER_CARDS_SPLIT: usize = 3;
const MAX_NUMBER_CARDS_SPLIT: usize = TOTAL_CARDS - MIN_NUMBER_CARDS_SPLIT;

pub trait GameActions {
    fn update_dealer(&mut self);
    fn split_deck(&mut self);
    fn collect_deck(&mut self, players: &[Player]);
    fn reorder_players(&mut self);
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub deals: Vec<Deal>,
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
            deals: Vec::new(),
        }
    }
}
impl GameActions for Game {
    fn split_deck(&mut self) {
        let split_index = random_int_in_range(1, MAX_NUMBER_CARDS_SPLIT);
        let mut new_deck = Vec::new();
        new_deck.extend_from_slice(&self.deck[split_index..]);
        new_deck.extend_from_slice(&self.deck[..split_index]);
        self.deck = new_deck;
    }

    fn collect_deck(&mut self, players: &[Player]) {
        let mut deck = Vec::new();
        for player in players {
            match player.hand.cards.len() > 0 {
                true => deck.extend(player.hand.cards.clone()),
                false => deck.extend(player.hand.won_cards.clone()),
            }
        }
        self.deck = deck;
    }
    fn update_dealer(&mut self) {
        let index = find_dealer(&self.players);
        self.players[index].is_dealer = false;

        let next_index = get_next_index(&self.players, index);
        self.players[next_index].is_dealer = true;

        println!("The dealer is {}", self.players[next_index].name);
    }
    fn reorder_players(&mut self) {
        let dealer_index = find_dealer(&self.players);
        let start_index = get_next_index(&self.players, dealer_index);
        let new_players = reorder(&self.players, start_index);
        self.players.clear();
        self.players.extend_from_slice(&new_players);
    }
}

// PLAYERS
// TODO: create a submodule game/players
fn generate_players(n_players: u8) -> Vec<Player> {
    let mut players = Vec::new();
    for i in 1..=n_players {
        let mut player = Player::new(format!("Player {i}"), i);
        player.is_human = i == 1;
        players.push(player);
    }
    players
}

fn set_first_dealer(players: &mut Vec<Player>) {
    let index = random_int_in_range(0, players.len());
    players[index].is_dealer = true;
}

fn create_players(n_players: u8) -> Vec<Player> {
    let mut players = generate_players(n_players);
    set_first_dealer(&mut players);
    players
}

fn generate_card(n_cards: usize, suit: CardSuits) -> Vec<Card> {
    let mut cards = Vec::new();
    for rank in 1..=n_cards {
        let card = Card::new(rank as u8, suit.clone());
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

pub fn find_dealer(players: &Vec<Player>) -> usize {
    players.iter().position(|player| player.is_dealer).unwrap()
}
