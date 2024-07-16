#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;

use std::fmt::Display;
use std::io;

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

#[derive(Debug, Clone)]
struct Player {
    name: String,
    score: u8,
    is_dealer: bool,
    is_bot: bool,
    cards: Vec<Card>,
}
impl Player {
    fn new(name: String, is_bot: bool) -> Player {
        Player {
            name,
            score: 0,
            is_dealer: false,
            is_bot,
            cards: Vec::new(),
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
    fn is_trump() -> bool;
    fn is_oudler(rank: u8) -> bool;
}

#[derive(Debug, Clone)]
struct Card {
    rank: u8,
    name: String,
    score: f64,
    suit: Suit,
    is_trump: bool,
    is_oudler: bool,
}
impl Card {
    fn new<T: CardGetters>(_: T, rank: u8, suit: Suit) -> Card {
        let score = T::score(rank);
        let name = T::name(rank);
        let is_trump = T::is_trump();
        let is_oudler = T::is_oudler(rank);
        Card {
            rank,
            name,
            score,
            suit,
            is_trump,
            is_oudler,
        }
    }
    fn id(&self) -> String {
        format!("|{} {}|", self.suit.icon, self.name)
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
    fn is_trump() -> bool {
        false
    }
    fn is_oudler(rank: u8) -> bool {
        match rank {
            1 | 21 | 22 => true,
            _ => false,
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
    fn is_trump() -> bool {
        true
    }
    fn is_oudler(_rank: u8) -> bool {
        false
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

// Score

fn compute_points(cards: &Vec<&Card>) -> f64 {
    cards.into_iter().fold(0.0, |acc, card| acc + card.score)
}

fn needed_points(n_oudlers: usize) -> f64 {
    match n_oudlers {
        0 => 56.0,
        1 => 51.0,
        2 => 41.0,
        3 => 36.0,
        _ => 0.0, // maybe raise an error
    }
}

// fn multiplier(bid: Bid) -> f64 {
//     match bid {
//         Bid::Petite => 1.0,
//         Bid::Garde => 2.0,
//         Bid::GardeSans => 4.0,
//         Bid::GardeContre => 6.0,
//     }
// }

fn compute_oudlers(cards: &Vec<&Card>) -> usize {
    cards.into_iter().filter(|card| card.is_oudler).count()
}

fn compute_needed_points(cards: &Vec<&Card>) -> f64 {
    needed_points(compute_oudlers(cards))
}

fn diff_points(cards: &Vec<&Card>) -> f64 {
    let points = compute_points(cards);
    let needed_points = compute_needed_points(cards);
    points - needed_points
}

// fn compute_score(hand: &Hand) -> f64 {
//     let points = diff_points(&hand.attack_pool);
//     let petit_au_bout = if hand.bonus_petit_au_bout { 10.0 } else { 0.0 };
//     ((25.0 + points + petit_au_bout) * multiplier(hand.bid))
//         + hand.bonus_poignee
//         + hand.bonus_chelem
// }

fn create_players(config: &Config) -> Vec<Player> {
    let mut players = Vec::new();
    for i in 1..=config.n_players {
        let player = Player::new(format!("Player {}", i), i != 1);
        players.push(player);
    }
    choose_first_dealer(&mut players);
    players
}

fn choose_first_dealer(players: &mut Vec<Player>) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..players.len());
    players[index].is_dealer = true;
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

fn reorder_players(players: &Vec<Player>) -> Vec<Player> {
    let start_index = get_player_index_after_dealer(players);
    let start = &players[start_index..];
    let end = &players[..start_index];
    [start, end].concat()
}

fn update_dealer(players: &mut Vec<Player>) {
    let index = players.iter().position(|player| player.is_dealer).unwrap();
    players[index].is_dealer = false;
    let new_index = get_next_player_index(players, index);
    players[new_index].is_dealer = true;
    println!("The dealer is {}", players[new_index].name);
}

fn deal_kitty_or_player(kitty: &Vec<Card>, kitty_expected_size: usize) -> String {
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
            players[player_index].cards.extend(split.to_vec());
            player_index = get_next_player_index(&players, player_index);
            index = end_of_range;
        } else {
            let end_of_range = index + DEAL_SIZE_KITTY;
            let split = &deck[index..end_of_range];
            kitty.extend(split.to_vec());
            index = end_of_range;
        }
        dealing_kitty_or_player = deal_kitty_or_player(&kitty, kitty_expected_size);
    }
}

fn display_cards(cards: &Vec<Card>) {
    println!("Cards in your hand:");
    for card in cards {
        print!("{} ", card);
    }
    println!();
}

fn auto_bid(cards: &Vec<Card>, previous_bid: &Bid) -> Bid {
    // Previous bids
    // Score of owned cards
    // let hand_score = compute_points(cards);
    let hand_score = 0.0;
    // println!("Hand score: {}", hand_score);
    match hand_score {
        0.0..=30.0 => Bid::Passe,
        31.0..=40.0 => Bid::Petite,
        41.0..=50.0 => Bid::Garde,
        _ => Bid::Passe,
    }
}

fn input_bid() -> Option<Bid> {
    println!("What is your bid?");
    println!("1. Petite, 2. Garde, 3. Garde Sans, 4. Garde Contre, 5. Passe");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read line failed.");
    let name = name.trim();
    match name {
        "1" | "petite" | "Petite" | "PETITE" => Some(Bid::Petite),
        "2" | "garde" | "Garde" | "GARDE" => Some(Bid::Garde),
        "3" | "garde-sans" | "GardeSans" | "GARDE-SANS" => Some(Bid::GardeSans),
        "4" | "garde-contre" | "GardeContre" | "GARDE-CONTRE" => Some(Bid::GardeContre),
        "5" | "passe" | "Passe" | "PASSE" => Some(Bid::Passe),
        _ => None,
    }
}

fn is_valid_bid(bid: &Bid, previous_bid: Option<&Bid>) -> bool {
    match previous_bid {
        Some(previous_bid) => match bid {
            Bid::Passe => true,
            Bid::Petite => [Bid::Passe].contains(&previous_bid),
            Bid::Garde => [Bid::Passe, Bid::Petite].contains(&previous_bid),
            Bid::GardeSans => [Bid::Passe, Bid::Petite, Bid::Garde].contains(&previous_bid),
            Bid::GardeContre => {
                [Bid::Passe, Bid::Petite, Bid::Garde, Bid::GardeSans].contains(&previous_bid)
            }
        },
        None => true,
    }
}

fn make_bid(cards: &Vec<Card>, previous_bid: &Bid) -> Bid {
    let bid = input_bid();
    match bid {
        Some(bid) => {
            if is_valid_bid(&bid, Some(previous_bid)) {
                println!("Your bid is {:?}", bid);
                bid
            } else {
                make_bid(cards, previous_bid)
            }
        }
        None => {
            println!("Type a number between 1 and 5");
            make_bid(cards, previous_bid)
        }
    }
}

fn collect_bid(players: &Vec<Player>) -> Taker {
    println!("Collecting bids...");
    let reordered_players = reorder_players(players);

    let mut taker = Taker {
        player: reordered_players[0].name.clone(),
        bid: Bid::Passe,
    };
    for player in reordered_players {
        let bid = match player.is_bot {
            true => auto_bid(&player.cards, &taker.bid),
            false => {
                display_cards(&player.cards);
                make_bid(&player.cards, &taker.bid)
            }
        };
        if is_valid_bid(&bid, Some(&taker.bid)) {
            taker.player = player.name.clone();
            taker.bid = bid;
        }
        println!("{}'s bid is {:?}", taker.player, taker.bid);
    }
    return taker;
}

#[derive(Debug, PartialEq)]
enum Bid {
    Petite,
    Garde,
    GardeSans,
    GardeContre,
    Passe,
}
// struct Poignee {}
// struct Chelem {}
//
struct Trick {
    cards_played: Vec<Card>,
}

#[derive(Debug)]
struct Taker {
    player: String,
    bid: Bid,
}

struct Hand {
    dealer: Player,
    bonus_petit_au_bout: bool,
    // bonus_poignee: Poignee,
    // bonus_chelem: Chelem,
    taker: Taker,
    tricks: Vec<Trick>,
    attack_pool: Vec<Card>,
    defense_pool: Vec<Card>,
    taker_oudlers: usize,
}

pub fn run(config: Config) {
    println!("There are {} players.", config.n_players);
    let mut deck = build_deck();
    // println!("Deck: {:?}", &deck);
    let mut players = create_players(&config);
    // println!("Players: {:?}", &players);
    let mut kitty = Vec::new();
    // println!("Kitty: {:?}", &kitty);
    // let mut hands = Vec::new();
    loop {
        // create a hand
        update_dealer(&mut players);
        split_deck(&mut deck);
        deal_cards(&deck, &mut players, &mut kitty);
        let taker = collect_bid(&players);
        println!("Taker: {:?}", &taker);
        if taker.bid == Bid::Passe {
            println!("Nobody made a bid. Starting a new hand...");
            continue;
        }

        //         // BID
        //         // First player (after the dealer) makes bid
        //         // - compute score in cardss if under X points PASS
        //         // - else if PETITE ...
        //         // while all players make their bid
        //         // - prompt bid for the player A
        //         // PLAY
        //         // Each player plays one card
        //         // - check which wins
        //         // MARK POINT
        //         // - compute score of taker
        //         // - check if taker wins
        //         // - store points
        break;
    }
}
