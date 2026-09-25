use rand::{rngs::StdRng, seq::SliceRandom, Rng, RngExt, SeedableRng};

use super::{
    card::{Card, CardSuits},
    player::{Player, PlayerActions, PlayerKind},
    utils::{get_next_index, read_input, reorder},
};

const NUMBER_CARDS_BY_SUIT: usize = 14;
const NUMBER_TRUMP_CARDS: usize = 22;

const TOTAL_CARDS: usize = 78;
const MIN_NUMBER_CARDS_SPLIT: usize = 3;
const MAX_NUMBER_CARDS_SPLIT: usize = TOTAL_CARDS - MIN_NUMBER_CARDS_SPLIT;

pub const MIN_PLAYERS: u8 = 3;
pub const MAX_PLAYERS: u8 = 5;
pub const DEFAULT_PLAYERS: u8 = 4;

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
    // Source of every random draw: shuffle, first dealer, cut and kitty positions
    pub rng: StdRng,
}
impl Default for Game {
    fn default() -> Self {
        Game::new(DEFAULT_PLAYERS)
    }
}
impl Game {
    pub fn new(n_players: u8) -> Self {
        Game::build(generate_players(n_players), rand::make_rng())
    }
    // All players are bots, and the same seed plays the same deals
    pub fn new_bots(n_players: u8, seed: u64) -> Self {
        let players = (1..=n_players)
            .map(|id| Player::new(format!("Bot {id}"), id, Some(PlayerKind::Bot)))
            .collect();
        Game::build(players, StdRng::seed_from_u64(seed))
    }
    fn build(mut players: Vec<Player>, mut rng: StdRng) -> Self {
        set_first_dealer(&mut players, &mut rng);
        let deck = create_deck(&mut rng);
        Game { players, deck, rng }
    }
}
impl GameActions for Game {
    fn split_deck(&mut self) {
        // Each part of the cut keeps more than 3 cards
        let split_index = self
            .rng
            .random_range(MIN_NUMBER_CARDS_SPLIT + 1..MAX_NUMBER_CARDS_SPLIT);
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

fn set_first_dealer(players: &mut [Player], rng: &mut impl Rng) {
    let index = rng.random_range(0..players.len());
    players[index].toggle_role();
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

pub fn create_deck(rng: &mut impl Rng) -> Vec<Card> {
    let mut deck = Vec::new();
    generate_suits(&mut deck);
    deck.shuffle(rng);
    deck.to_vec()
}

pub fn find_dealer(players: &[Player]) -> usize {
    players
        .iter()
        .position(|player| player.is_dealer())
        .unwrap()
}

// Reads a player count between MIN_PLAYERS and MAX_PLAYERS. Empty input picks DEFAULT_PLAYERS.
pub fn parse_player_count(input: &str) -> Option<u8> {
    let input = input.trim();
    if input.is_empty() {
        return Some(DEFAULT_PLAYERS);
    }
    input
        .parse::<u8>()
        .ok()
        .filter(|n| (MIN_PLAYERS..=MAX_PLAYERS).contains(n))
}

pub fn ask_player_count() -> u8 {
    loop {
        println!(
            "How many players? ({MIN_PLAYERS} to {MAX_PLAYERS}, press Enter for {DEFAULT_PLAYERS})"
        );
        match parse_player_count(&read_input()) {
            Some(n_players) => return n_players,
            None => println!("Please enter a number from {MIN_PLAYERS} to {MAX_PLAYERS}."),
        }
    }
}
