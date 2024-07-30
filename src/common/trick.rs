use super::card::Card;

#[derive(Debug)]
pub struct Trick {
    cards_played: Vec<Card>,
}

impl Trick {
    pub fn new() -> Self {
        Trick {
            cards_played: Vec::new(),
        }
    }
}
