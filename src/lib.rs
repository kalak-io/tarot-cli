#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;

use std::fmt::Display;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

const NUMBER_CARDS: u8 = 78;
const MIN_NUMBER_CARDS_SPLIT: u8 = 3;
const MAX_NUMBER_CARDS_SPLIT: u8 = NUMBER_CARDS - MIN_NUMBER_CARDS_SPLIT;
const DEAL_SIZE_PLAYERS: usize = 3;
const DEAL_SIZE_KITTY: usize = 1;

#[derive(Debug)]
pub struct Config {
    n_players: u8,
}
impl Default for Config {
    fn default() -> Config {
        Config { n_players: 4_u8 }
    }
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let len = args.len();
        match len {
            1 => Ok(Config::default()),
            2 => match args[1].clone().trim().parse::<u8>() {
                Ok(n_players) => match n_players {
                    2 | 3 | 4 | 5 => Ok(Config { n_players }),
                    _ => Err("Invalid number of players"),
                },
                Err(_) => Err("Could not parse the number of players"),
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    score: u8,
    is_dealer: bool,
    hand: Vec<Card>,
}
impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            is_dealer: false,
            hand: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum CardSuits {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    Trumps,
}

#[derive(Debug, Clone)]
struct Suit {
    name: String,
    icon: char,
}
impl Suit {
    fn new(name: CardSuits) -> Suit {
        match name {
            CardSuits::Spades => Suit {
                name: String::from("Spades"),
                icon: '♠',
            },
            CardSuits::Hearts => Suit {
                name: String::from("Hearts"),
                icon: '♥',
            },
            CardSuits::Diamonds => Suit {
                name: String::from("Diamonds"),
                icon: '♦',
            },
            CardSuits::Clubs => Suit {
                name: String::from("Clubs"),
                icon: '♣',
            },
            CardSuits::Trumps => Suit {
                name: String::from("Trumps"),
                icon: '*',
            },
        }
    }
}

trait CardGetters {
    fn score(rank: u8) -> f64;
    fn name(rank: u8) -> String;
}

#[derive(Debug, Clone)]
struct Card {
    rank: u8,
    name: String,
    score: f64,
    suit: Suit,
}
impl Card {
    fn new<T: CardGetters>(_: T, rank: u8, suit: Suit) -> Card {
        let score = T::score(rank);
        let name = T::name(rank);
        Card {
            rank,
            name,
            score,
            suit,
        }
    }
    fn is_trump(&self) -> bool {
        self.suit.name == "Trumps"
    }
    fn id(&self) -> String {
        format!("|{}{}|", self.suit.icon, self.name)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id())
    }
}

#[derive(Copy, Clone)]
struct CardSuit;
impl CardGetters for CardSuit {
    fn score(rank: u8) -> f64 {
        match rank {
            1..=10 => 0.5,
            11 => 1.5,
            12 => 2.5,
            13 => 3.5,
            14 => 4.5,
            _ => 0.0,
        }
    }
    fn name(rank: u8) -> String {
        match rank {
            11 => String::from("Jack"),
            12 => String::from("Knight"),
            13 => String::from("Queen"),
            14 => String::from("King"),
            _ => rank.to_string(),
        }
    }
}

#[derive(Copy, Clone)]
struct CardTrump;
impl CardGetters for CardTrump {
    fn score(rank: u8) -> f64 {
        match rank {
            1 | 21 | 22 => 4.5,
            _ => 0.5,
        }
    }
    fn name(rank: u8) -> String {
        match rank {
            1 => String::from("Petit"),
            22 => String::from("Joker"),
            _ => rank.to_string(),
        }
    }
}

fn _generate_cards<const N: usize, T: CardGetters + std::marker::Copy>(
    card_type: T,
    n_cards: u8,
    suits: [Suit; N],
) -> Vec<Card> {
    let mut cards = Vec::new();
    for suit in suits {
        for rank in 1..=n_cards {
            let card = Card::new(card_type, rank, suit.clone());
            cards.push(card);
        }
    }
    cards.to_vec()
}

fn build_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    let suits: [Suit; 4] = [
        Suit::new(CardSuits::Spades),
        Suit::new(CardSuits::Hearts),
        Suit::new(CardSuits::Diamonds),
        Suit::new(CardSuits::Clubs),
    ];
    deck.extend(_generate_cards(CardSuit, 14, suits));
    let trumps: [Suit; 1] = [Suit::new(CardSuits::Trumps)];
    deck.extend(_generate_cards(CardTrump, 22, trumps));
    deck.shuffle(&mut thread_rng());
    deck.to_vec()
}

fn split_deck(deck: &mut Vec<Card>) {
    let split_index = rand::thread_rng().gen_range(1..MAX_NUMBER_CARDS_SPLIT) as usize;
    let head = &deck[..split_index];
    let tail = &deck[split_index..];
    *deck = [tail, head].concat();
}

fn kitty_expected_size(n_players: usize) -> usize {
    match n_players {
        2..=4 => 6,
        5.. => 3,
        _ => 0, // maybe raise an error
    }
}

fn compute_score(cards: &Vec<&Card>) -> f64 {
    cards.into_iter().fold(0.0, |acc, card| acc + card.score)
}

fn create_players(config: &Config) -> Vec<Player> {
    let mut players = Vec::new();
    for i in 1..=config.n_players {
        let player = Player::new(format!("Player {}", i));
        players.push(player);
    }
    choose_first_dealer(&mut players);
    players
}

fn choose_first_dealer(players: &mut Vec<Player>) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..players.len());
    players[index].is_dealer = true;
    println!("The dealer is {}", players[index].name);
}

fn get_player_index_after_dealer(players: &Vec<Player>) -> usize {
    let index = players.iter().position(|player| player.is_dealer).unwrap();
    get_next_player_index(players, index) as usize
}

fn get_next_player_index(players: &Vec<Player>, current_index: usize) -> usize {
    if current_index == players.len() - 1 {
        0
    } else {
        current_index + 1
    }
}

fn update_dealer(players: &mut Vec<Player>) {
    let index = players.iter().position(|player| player.is_dealer).unwrap();
    players[index].is_dealer = false;
    let new_index = get_next_player_index(players, index);
    players[new_index].is_dealer = true;
    println!("The dealer is {}", players[new_index].name);
}

fn deal_kitty_or_player(
    kitty: &Vec<Card>,
    kitty_expected_size: usize,
    remaining_cards: usize,
) -> String {
    if kitty.len() < kitty_expected_size {
        return ["player".to_string(), "kitty".to_string()]
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
    } else {
        return "player".to_string();
    }
}

fn deal_cards(deck: &Vec<Card>, players: &mut Vec<Player>, kitty: &mut Vec<Card>) {
    let mut index: usize = 0;
    let mut dealing_kitty_or_player = "player".to_string();
    let mut player_index = get_player_index_after_dealer(players);
    let kitty_expected_size = kitty_expected_size(players.len());
    while index < deck.len() {
        if dealing_kitty_or_player == "player" {
            let end_of_range = index + DEAL_SIZE_PLAYERS;
            let split = &deck[index..end_of_range];
            players[player_index].hand.extend(split.to_vec());
            player_index = get_next_player_index(&players, player_index);
            index = end_of_range;
        } else {
            let end_of_range = index + DEAL_SIZE_KITTY;
            let split = &deck[index..end_of_range];
            kitty.extend(split.to_vec());
            index = end_of_range;
        }
        dealing_kitty_or_player =
            deal_kitty_or_player(&kitty, kitty_expected_size, deck.len() - index);
    }
}

pub fn run(config: Config) {
    println!("There are {} players.", config.n_players);
    let mut deck = build_deck();
    let mut players = create_players(&config);
    let mut kitty = Vec::new();
    loop {
        update_dealer(&mut players);
        split_deck(&mut deck);
        deal_cards(&deck, &mut players, &mut kitty);
        println!("{}'s hand is {:?}", players[0].name, players[0].hand);
        break;
    }
}
