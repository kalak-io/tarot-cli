use super::{bid::Bids, player::Player};

#[derive(Debug, Clone, Default)]
pub struct Taker {
    pub player: Player,
    pub bid: Bids,
}
