use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Trumps,
}

#[derive(Debug, Clone)]
pub struct Suit {
    pub name: String,
    icon: char,
    initial: char,
}
impl Suit {
    pub fn new(name: CardSuits) -> Suit {
        match name {
            CardSuits::Clubs => Suit {
                name: String::from("Clubs"),
                icon: '♣',
                initial: 'C',
            },
            CardSuits::Diamonds => Suit {
                name: String::from("Diamonds"),
                icon: '♦',
                initial: 'D',
            },
            CardSuits::Spades => Suit {
                name: String::from("Spades"),
                icon: '♠',
                initial: 'S',
            },
            CardSuits::Hearts => Suit {
                name: String::from("Hearts"),
                icon: '♥',
                initial: 'H',
            },
            CardSuits::Trumps => Suit {
                name: String::from("Trumps"),
                icon: '*',
                initial: 'T',
            },
        }
    }
}

pub trait CardGetters {
    fn score(rank: u8) -> f64;
    fn name(rank: u8) -> String;
    fn is_trump() -> bool;
    fn is_oudler(rank: u8) -> bool;
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: String,
    rank: u8,
    name: String,
    pub score: f64,
    pub suit: Suit,
    pub is_trump: bool,
    pub is_oudler: bool,
}
impl Card {
    pub fn new<T: CardGetters>(_: T, rank: u8, suit: Suit) -> Self {
        let id = format!("{}{}", suit.initial, rank);
        let score = T::score(rank);
        let name = T::name(rank);
        let is_trump = T::is_trump();
        let is_oudler = T::is_oudler(rank);
        Card {
            id,
            rank,
            name,
            score,
            suit,
            is_trump,
            is_oudler,
        }
    }
    pub fn is_superior_than(self, card: &Card) -> bool {
        match self.is_trump {
            true => match card.is_trump {
                true => self.rank > card.rank,
                false => true,
            },
            false => match card.is_trump {
                true => false,
                false => self.rank > card.rank,
            },
        }
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{} {}| ", self.suit.icon, self.name)
    }
}

#[derive(Copy, Clone)]
pub struct CardSuit;
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
    fn is_oudler(_rank: u8) -> bool {
        false
    }
}

#[derive(Copy, Clone)]
pub struct CardTrump;
impl CardGetters for CardTrump {
    fn score(rank: u8) -> f64 {
        match rank {
            1 | 21 | 22 => 4.5,
            _ => 0.5,
        }
    }
    fn name(rank: u8) -> String {
        match rank {
            22 => String::from("Fool"),
            _ => rank.to_string(),
        }
    }
    fn is_trump() -> bool {
        true
    }
    fn is_oudler(rank: u8) -> bool {
        match rank {
            1 | 21 | 22 => true,
            _ => false,
        }
    }
}
