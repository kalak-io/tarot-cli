use super::card::Card;

#[derive(Debug, Default)]
pub struct Trick {
    cards_played: Vec<Card>,
}
