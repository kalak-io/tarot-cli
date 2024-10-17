use std::fmt::{Display, Formatter, Result};

pub const KING_RANK: u8 = 14;
const QUEEN_RANK: u8 = 13;
const KNIGHT_RANK: u8 = 12;
const JACK_RANK: u8 = 11;
const LITTLE_RANK: u8 = 1;
const BIG_RANK: u8 = 21;
const FOOL_RANK: u8 = 22;

pub trait CardSuitsGetters {
    fn is_trump(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Trumps,
}
impl Display for CardSuits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
impl CardSuits {
    pub const AVAILABLE_SUITS: [Self; 5] = [
        Self::Clubs,
        Self::Diamonds,
        Self::Hearts,
        Self::Spades,
        Self::Trumps,
    ];
}
impl CardSuitsGetters for CardSuits {
    fn is_trump(&self) -> bool {
        matches!(self, CardSuits::Trumps)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Suit {
    pub name: CardSuits,
    pub icon: char,
    pub initial: char,
}

impl Suit {
    pub fn new(name: CardSuits) -> Suit {
        let (icon, initial) = get_suit_data(name);
        Suit {
            name,
            icon,
            initial,
        }
    }
}
impl CardSuitsGetters for Suit {
    fn is_trump(&self) -> bool {
        matches!(self.name, CardSuits::Trumps)
    }
}

pub trait CardGetters {
    fn is_oudler(&self) -> bool;
    fn score(&self) -> f64;
    fn name(&self) -> String;
    fn id(&self) -> String;
}

pub trait CardActions {
    fn is_superior_than(&self, card: &Card, played_suit: Option<CardSuits>) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}
impl Card {
    pub fn new(rank: u8, suit: CardSuits) -> Self {
        let suit = Suit::new(suit);
        Card { rank, suit }
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, " | {} {} | ", self.name(), self.suit.icon)
    }
}
impl CardGetters for Card {
    fn score(&self) -> f64 {
        match (self.rank, self.suit.name) {
            (LITTLE_RANK | BIG_RANK | FOOL_RANK, CardSuits::Trumps) => 4.5,
            (2..BIG_RANK, CardSuits::Trumps) => 0.5,
            (KING_RANK, _) => 4.5,
            (QUEEN_RANK, _) => 3.5,
            (KNIGHT_RANK, _) => 2.5,
            (JACK_RANK, _) => 1.5,
            (1..=10, _) => 0.5,
            (_, _) => 0.0,
        }
    }
    fn name(&self) -> String {
        match (self.rank, self.suit.name) {
            (FOOL_RANK, CardSuits::Trumps) => String::from("Fool"),
            (KING_RANK, CardSuits::Clubs)
            | (KING_RANK, CardSuits::Spades)
            | (KING_RANK, CardSuits::Hearts)
            | (KING_RANK, CardSuits::Diamonds) => String::from("King"),
            (QUEEN_RANK, CardSuits::Clubs)
            | (QUEEN_RANK, CardSuits::Spades)
            | (QUEEN_RANK, CardSuits::Hearts)
            | (QUEEN_RANK, CardSuits::Diamonds) => String::from("Queen"),
            (KNIGHT_RANK, CardSuits::Clubs)
            | (KNIGHT_RANK, CardSuits::Spades)
            | (KNIGHT_RANK, CardSuits::Hearts)
            | (KNIGHT_RANK, CardSuits::Diamonds) => String::from("Knight"),
            (JACK_RANK, CardSuits::Clubs)
            | (JACK_RANK, CardSuits::Spades)
            | (JACK_RANK, CardSuits::Hearts)
            | (JACK_RANK, CardSuits::Diamonds) => String::from("Jack"),
            (_, _) => self.rank.to_string(),
        }
    }
    fn id(&self) -> String {
        format!("{}{}", self.suit.initial, self.rank)
    }
    fn is_oudler(&self) -> bool {
        match (self.rank, self.suit.name) {
            (LITTLE_RANK | BIG_RANK | FOOL_RANK, CardSuits::Trumps) => true,
            (_, _) => false,
        }
    }
}

impl CardActions for Card {
    fn is_superior_than(&self, card: &Card, played_suit: Option<CardSuits>) -> bool {
        match played_suit {
            Some(played_suit) => match (self.suit.name, card.suit.name, played_suit) {
                (CardSuits::Trumps, _, __) => {
                    if card.suit.name.is_trump() {
                        self.rank > card.rank
                    } else {
                        true
                    }
                }
                (_, CardSuits::Trumps, __) => false,
                _ => {
                    if self.suit.name == played_suit && card.suit.name == played_suit {
                        self.rank > card.rank
                    } else if self.suit.name == played_suit && card.suit.name != played_suit {
                        true
                    } else if self.suit.name != played_suit && card.suit.name == played_suit {
                        false
                    } else {
                        false
                    }
                }
            },
            None => {
                if self.suit.name.is_trump() && card.suit.name.is_trump() {
                    self.rank > card.rank
                } else if self.suit.name.is_trump() {
                    true
                } else if card.suit.name.is_trump() {
                    false
                } else {
                    self.rank > card.rank
                }
            }
        }
    }
}

fn get_suit_data(name: CardSuits) -> (char, char) {
    match name {
        CardSuits::Clubs => ('♣', 'C'),
        CardSuits::Diamonds => ('♦', 'D'),
        CardSuits::Spades => ('♠', 'S'),
        CardSuits::Hearts => ('♥', 'H'),
        CardSuits::Trumps => ('*', 'T'),
    }
}
