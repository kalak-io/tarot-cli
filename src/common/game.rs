use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::card::{Card, CardGetters, CardSuit, CardSuits, CardTrump, Suit};
use super::deal::{toggle_is_dealer, Deal};
use super::player::Player;
use super::utils::random_int_in_range;

const NUMBER_CARDS_BY_SUIT: usize = 14;
const NUMBER_TRUMP_CARDS: usize = 22;

const TOTAL_CARDS: usize = 78;
const MIN_NUMBER_CARDS_SPLIT: usize = 3;
const MAX_NUMBER_CARDS_SPLIT: usize = TOTAL_CARDS - MIN_NUMBER_CARDS_SPLIT;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub deals: Vec<Deal>,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            players: create_players(4_u8),
            deck: create_deck(),
            deals: Vec::new(),
        }
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
    toggle_is_dealer(players, index);
}

fn create_players(n_players: u8) -> Vec<Player> {
    let mut players = generate_players(n_players);
    set_first_dealer(&mut players);
    players
}

// DECK
// TODO: create a submodule game/deck
fn generate_cards<const N: usize, T: CardGetters + std::marker::Copy>(
    card_type: T,
    n_cards: usize,
    suits: [Suit; N],
) -> Vec<Card> {
    let mut cards = Vec::new();
    for suit in suits {
        for rank in 1..=n_cards {
            let card = Card::new(card_type, rank as u8, suit.clone());
            cards.push(card);
        }
    }
    cards.to_vec()
}

fn generate_suit_cards(deck: &mut Vec<Card>) {
    let suits: [Suit; 4] = [
        Suit::new(CardSuits::Spades),
        Suit::new(CardSuits::Hearts),
        Suit::new(CardSuits::Diamonds),
        Suit::new(CardSuits::Clubs),
    ];
    deck.extend(generate_cards(CardSuit, NUMBER_CARDS_BY_SUIT, suits))
}

fn generate_trump_cards(deck: &mut Vec<Card>) {
    let trumps: [Suit; 1] = [Suit::new(CardSuits::Trumps)];
    deck.extend(generate_cards(CardTrump, NUMBER_TRUMP_CARDS, trumps));
}

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    generate_suit_cards(&mut deck);
    generate_trump_cards(&mut deck);
    deck.shuffle(&mut thread_rng());
    deck.to_vec()
}

pub fn split_deck(deck: &mut Vec<Card>) {
    let split_index = random_int_in_range(1, MAX_NUMBER_CARDS_SPLIT);
    let head = &deck[..split_index];
    let tail = &deck[split_index..];
    *deck = [tail, head].concat();
}

pub fn collect_deck(players: &[Player]) -> Vec<Card> {
    let mut deck = Vec::new();
    for player in players {
        match player.cards.len() > 0 {
            true => deck.extend(player.cards.clone()),
            false => deck.extend(player.picked_up_cards.clone()),
        }
    }
    deck
}
