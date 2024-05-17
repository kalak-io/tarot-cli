#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;

use rand::seq::SliceRandom;
use rand::thread_rng;

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
}
impl Player {
    fn new(name: String) -> Player {
        Player { name, score: 0 }
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

fn kitty_size(n_players: u8) -> u8 {
    match n_players {
        2..=4 => 6,
        5..=6 => 3,
        7.. => 3,
        _ => 0, // maybe raise an error
    }
}

fn draw_cards(deck: &mut Vec<Card>, players: &mut Vec<Player>) {
    // slice deck between all players and the kitty
    // if 3 or 4 players, the kitty length is 6 cards
    // if 5 or 6 players, the kitty length is 3 cards
}

fn compute_score(cards: &Vec<&Card>) -> f64 {
    cards.into_iter().fold(0.0, |acc, card| acc + card.score)
}

pub fn run(config: Config) {
    dbg!(&config);
    println!("There are {} players.", config.n_players);
    let deck = build_deck();
    println!("Deck: {:?}", deck);
}
