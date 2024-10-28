use super::card::Card;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Side {
    Attack,
    #[default]
    Defense,
}

pub trait HandActions {
    fn set_side_with_called_king(&mut self, called_king: Option<Card>);
}
#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub won_cards: Vec<Card>,
    pub side: Side,
}

impl HandActions for Hand {
    fn set_side_with_called_king(&mut self, called_king: Option<Card>) {
        if let Some(called_king) = called_king {
            if self.cards.contains(&called_king) {
                self.side = Side::Attack;
            }
        }
    }
}
