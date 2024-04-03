use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{self, Rng};

const SUIT_NAMES: [&str; 4] = ["Spades", "Hearts", "Diamonds", "Clubs"];
const SUIT_ICONS: [char; 4] = ['♠', '♥', '♦', '♣'];

const PLAYER_NAMES: [char; 5] = ['A', 'B', 'C', 'D', 'E'];

const MIN_NUMBER_CARDS_SPLIT: u8 = 3;
const MAX_NUMBER_CARDS_SPLIT: u8 = 78 - MIN_NUMBER_CARDS_SPLIT;

#[derive(Clone, Copy, Debug)]
pub struct Suit<'a> {
    name: &'a str,
    icon: &'a char,
}

impl Suit<'_> {
    fn show(&self) {
        println!("#####");
        println!("{} {}", self.icon, self.name);
    }
}

#[derive(Debug, Clone)]
pub struct Card<'a> {
    rank: u8,
    suit: Option<Suit<'a>>,
}

impl Card<'_> {
    fn is_trump(&self) -> bool {
        !SUIT_NAMES.contains(&self.suit_name())
    }
    fn suit_name(&self) -> &str {
        match self.suit {
            None => "",
            Some(suit) => &suit.name,
        }
    }
    fn suit_icon(&self) -> &char {
        match self.suit {
            None =>  &' ',
            Some(suit) => suit.icon,
        }
    }
    fn score(&self) -> f64 {
        if self.is_trump() {
            match self.rank {
                1 | 21 | 22 => 4.5,
                _ => 0.5
            }
        } else {
            match self.rank {
              1..=10 => 0.5,
              11 => 1.5,
              12 => 2.5,
              13 => 3.5,
              14 => 4.5,
              _ => 0.0
            }
        }
    }
    fn name(&self) -> String {
        if self.is_trump() {
            match self.rank {
                1 => String::from("Petit"),
                22 => String::from("Excuse"),
                _ => String::from(self.rank.to_string()),
            }
        } else {
            match self.rank {
                11 => String::from("Jack"),
                12 => String::from("Knight"),
                13 => String::from("Queen"),
                14 => String::from("King"),
                _ => String::from(self.rank.to_string()),
            }
        }
    }
    fn id(&self) -> String {
        format!("{}{}", self.suit_icon(), self.name())
    }
    fn show(&self) {
        println!("{}", self.id());
    }
}

pub fn compute_score(cards: &Vec<Card>) -> f64 {
    let mut score = 0.0;
    for card in cards {
        score += card.score();
    }
    score
}
#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/compute_score.rs"]
mod compute_score;

#[derive(Debug)]
pub struct Game<'a> {
    pub deck: Deck<'a>,
    players: Vec<Player<'a>>,
    dealer: u8, // index of the dealer
    hands: Option<Vec<Hand<'a>>>, // means when a player make a bid
}
impl Game<'_> {
    pub fn new(n_players: u8) -> Game<'static> {
        let mut players = vec![];
        for i in 0..n_players {
            players.push(
                Player::new(PLAYER_NAMES[i as usize]),
            );
        }

        let mut deck = Deck::new();
        deck.shuffle();

        let dealer = rand::thread_rng().gen_range(0..players.len()) as u8;

        Game {
            deck,
            players,
            dealer,
            hands: None,
        }
    }
    pub fn update_dealer(&mut self) {
        self.dealer = (self.dealer + 1) % self.players.len() as u8;
    }
    pub fn reorder_players(&mut self) {
        let next_player_index = (self.dealer + 1) % self.players.len() as u8;
        let first_players = &self.players[next_player_index as usize ..];
        let second_players = &self.players[..next_player_index as usize];
        self.players = [first_players, second_players].concat();
    }
}

#[derive(Debug)]
struct Hand<'a> {
    taker: Player<'a>,
    called: Option<Player<'a>>,
    defense: Vec<Player<'a>>,
    //bid: Bid,
    //annonces
    tricks: Vec<Trick<'a>>,
    kitty: Vec<Card<'a>>,
    //dead_players: Vec<Player<'a>>,
}


//enum Bid {
//    Small,
//    Guard,
//    GuardWithoutTheKitty,
//    GuardAgainstTheKitty,
//}

#[derive(Debug)]
struct Trick<'a> {
    cards: Vec<Card<'a>>,
}

#[derive(Debug, Clone)]
pub struct Player<'a> {
    name: char,
    deck: Option<Deck<'a>>,
    is_dealer: bool,
}
impl Player<'_> {
    fn new(name: char) -> Player<'static> {
        Player {
            name,
            deck: None,
            is_dealer: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck<'a> {
    cards: Vec<Card<'a>>,
}
impl Deck<'_> {
    fn new() -> Deck<'static> {
        let mut cards = vec![];
        for (suit_name, suit_icon) in SUIT_NAMES.iter().zip(SUIT_ICONS.iter()) {
             let suit = Suit{ name: suit_name, icon: suit_icon };
             for rank in 1..15 {
                 let card = Card { rank: rank, suit: Some(suit.clone()) };
                 cards.push(card);
             }
        }
        for rank in 1..23 {
            let card = Card { rank: rank, suit: None };
            cards.push(card);
        }
        println!("{:?}", cards);
        let shuffled_cards = cards.shuffle(&mut thread_rng());
        println!("---INIT DECK---");
        println!("{:?}", shuffled_cards);
        Deck { cards }
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    fn _show(&self) {
        for card in &self.cards {
            card.show();
        }
    }
    pub fn split(&mut self) {
        // The pack of cards must be cut in two, taking or leaving must have more than 3 cards.
        let split_index = rand::thread_rng().gen_range(0..MAX_NUMBER_CARDS_SPLIT);
        let first_slice = &self.cards[..split_index as usize];
        let second_slice = &self.cards[split_index as usize ..];
        self.cards = [second_slice, first_slice].concat();
    }
}

#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/deck.rs"]
mod desk;
