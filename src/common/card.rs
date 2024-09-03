use std::fmt::{Display, Formatter, Result};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Suit {
    pub name: CardSuits,
    pub icon: char,
    pub initial: char,
}

impl Suit {
    pub fn new(name: CardSuits) -> Suit {
        let suit_data = match name {
            CardSuits::Clubs => ('♣', 'C'),
            CardSuits::Diamonds => ('♦', 'D'),
            CardSuits::Spades => ('♠', 'S'),
            CardSuits::Hearts => ('♥', 'H'),
            CardSuits::Trumps => ('*', 'T'),
        };
        match suit_data {
            (icon, initial) => Suit {
                name,
                icon,
                initial,
            },
        }
    }
}

pub trait CardGetters {
    fn is_trump(&self) -> bool;
    fn is_oudler(&self) -> bool;
    fn score(&self) -> f64;
    fn name(&self) -> String;
    fn id(&self) -> String;
}

pub trait CardActions {
    fn is_superior_than(&self, card: &Card) -> bool;
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
        write!(f, "{} of {}", self.name(), self.suit.icon)
    }
}
impl CardGetters for Card {
    fn score(&self) -> f64 {
        if self.is_trump() {
            match self.rank {
                1 | 21 | 22 => 4.5,
                _ => 0.5,
            }
        } else {
            match self.rank {
                1..=10 => 0.5,
                11 => 1.5,
                12 => 2.5,
                13 => 3.5,
                14 => 4.5,
                _ => 0.0,
            }
        }
    }
    fn name(&self) -> String {
        if self.is_trump() {
            match self.rank {
                22 => String::from("Fool"),
                _ => self.rank.to_string(),
            }
        } else {
            match self.rank {
                11 => String::from("Jack"),
                12 => String::from("Knight"),
                13 => String::from("Queen"),
                14 => String::from("King"),
                _ => self.rank.to_string(),
            }
        }
    }
    fn id(&self) -> String {
        format!("{}{}", self.suit.initial, self.rank)
    }
    fn is_trump(&self) -> bool {
        match self.suit.name {
            CardSuits::Trumps => true,
            _ => false,
        }
    }
    fn is_oudler(&self) -> bool {
        if self.is_trump() {
            match self.rank {
                1 | 21 | 22 => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl CardActions for Card {
    fn is_superior_than(&self, card: &Card) -> bool {
        if self.is_trump() {
            match card.is_trump() {
                true => self.rank > card.rank,
                false => true,
            }
        } else {
            match card.is_trump() {
                true => false,
                false => self.rank > card.rank,
            }
        }
    }
}
