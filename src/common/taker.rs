use super::{bid::Bid, player::Player};

#[derive(Debug, Clone, Default)]
pub struct Taker {
    pub player: Player,
    pub bid: Bid,
}
