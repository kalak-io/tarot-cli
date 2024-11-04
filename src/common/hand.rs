use super::card::Card;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Side {
    Attack,
    #[default]
    Defense,
}
#[derive(Debug, Clone)]
pub enum Poignee {
    Simple,
    Double,
    Triple,
}

#[derive(Debug, Clone)]
pub enum Chelem {
    AnnouncedAndSucceed,
    AnnouncedAndLost,
    NotAnnouncedAndSucceed,
}

pub trait HandActions {
    fn set_side_with_called_king(&mut self, called_king: Option<Card>);
    fn human_declare_poignee(&mut self);
    fn human_declare_chelem(&mut self);
    fn bot_declare_poignee(&mut self);
    fn bot_declare_chelem(&mut self);
}
#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub won_cards: Vec<Card>,
    pub side: Side,
    pub bonus_poignee: Option<Poignee>,
    pub bonus_chelem: Option<Chelem>,
}

impl HandActions for Hand {
    fn set_side_with_called_king(&mut self, called_king: Option<Card>) {
        if let Some(called_king) = called_king {
            if self.cards.contains(&called_king) {
                self.side = Side::Attack;
            }
        }
    }
    fn human_declare_poignee(&mut self) {
        todo!()
    }
    fn human_declare_chelem(&mut self) {
        todo!()
    }
    fn bot_declare_poignee(&mut self) {
        todo!()
    }
    fn bot_declare_chelem(&mut self) {
        todo!()
    }
}
