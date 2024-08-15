use super::card::Card;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
enum Side {
    Attack,
    #[default]
    Defense,
}

#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub won_cards: Vec<Card>,
    side: Side,
}
