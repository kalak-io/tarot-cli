use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::common::utils::display_cards;

use super::{
    bid::{Bid, Bids},
    card::{count_cards_by_hand, Card},
    chelem::{Chelem, ChelemResult, ChelemState},
    hand::{Hand, Poignee, Side},
    kitty::Kitty,
    player::{Player, PlayerActions},
    score::compute_score,
    taker::Taker,
    trick::{Trick, TrickActions, TrickGetters},
    utils::{get_next_index, reorder},
};

const DEAL_SIZE_PLAYERS: usize = 3;
const DEAL_SIZE_KITTY: usize = 1;

pub trait DealActions {
    fn take_bids(&mut self);
    fn take_chelem(&mut self);
    fn call_king(&mut self);
    fn compose_kitty(&mut self);
    fn play_tricks(&mut self);
    fn set_side(&mut self);
    fn set_score(&mut self);
    fn show_score(&self);
}

pub trait DealGetters {
    fn bonus_petit_au_bout(&self) -> Option<Side>;
}
#[derive(Debug, Default)]
pub struct Deal {
    pub kitty: Kitty,
    pub players: Vec<Player>,
    pub taker: Option<Taker>,
    pub tricks: Vec<Trick>,
    pub called_king: Option<Card>,
}
impl Deal {
    pub fn new(players: &mut Vec<Player>, deck: &mut [Card]) -> Self {
        let mut kitty = Kitty::new(players.len());
        draw_cards(deck, players, &mut kitty);

        Deal {
            players: players.to_vec(), // TODO: is it necessary ?
            kitty,
            ..Default::default()
        }
    }
}
impl DealActions for Deal {
    fn take_bids(&mut self) {
        let mut bid = Bid::default();
        self.taker = collect_bids(&self.players, self.taker.clone(), &mut bid);
    }
    fn take_chelem(&mut self) {
        if let Some(taker) = &mut self.taker {
            taker.player.declare_chelem();
        } else {
            for player in &mut self.players {
                player.declare_chelem();
            }
        }
    }
    fn call_king(&mut self) {
        if self.players.len() > 4 {
            self.called_king = Some(self.taker.clone().unwrap().player.call_king());
            println!("\nThe called king is {}", self.called_king.unwrap());
        }
    }
    fn compose_kitty(&mut self) {
        let bid = self.taker.as_ref().unwrap().bid;
        match bid {
            Bids::GuardWithout | Bids::GuardAgainst => {
                println!("\n\nThe kitty stays hidden");
            }
            _ => {
                println!("\n\nThe kitty contains: ");
                display_cards(&self.kitty.cards);
                let taker_id = self.taker.as_ref().unwrap().player.id;
                let taker_index = self.players.iter().position(|p| p.id == taker_id).unwrap();
                // Temporarily take kitty out to satisfy borrow checker
                let mut kitty = std::mem::take(&mut self.kitty);
                self.players[taker_index].compose_kitty(&mut kitty);
                self.kitty = kitty;
            }
        }
    }
    fn play_tricks(&mut self) {
        if self.players[0].hand.cards.is_empty() {
            // set_bonus_petit_au_bout()
            return;
        }

        let n_players = self.players.len();
        let mut trick = Trick::default();
        for player in &mut self.players {
            if count_cards_by_hand(n_players) == player.hand.cards.len() as u8 {
                player.declare_poignee();
            }
            player.play(&mut trick);
        }
        let winner_index = trick.get_best_played_card_index(trick.played_suit());
        self.players[winner_index.unwrap()]
            .hand
            .won_cards
            .extend(trick.played_cards.clone());
        trick.winner_side = self.players[winner_index.unwrap()].hand.side;
        self.players = reorder(&self.players, winner_index.unwrap());
        self.tricks.push(trick);
        self.play_tricks()
    }
    fn set_side(&mut self) {
        // Set called-king holder's side (5-player game)
        if self.called_king.is_some() {
            for player in &mut self.players {
                player.hand.set_side_with_called_king(self.called_king);
            }
        }
        // Set the taker's side directly in self.players (taker.player is a clone)
        if let Some(taker) = &self.taker {
            let taker_id = taker.player.id;
            for player in &mut self.players {
                if player.id == taker_id {
                    player.hand.side = Side::Attack;
                    break;
                }
            }
        }
    }
    fn set_score(&mut self) {
        let won_cards_by_attack = merge_won_cards(&self.players);
        let bonus_chelem = compute_chelem_result(&self.tricks, &self.players);
        let bonus_poignee = best_poignee(&self.players);
        let attack_score = compute_score(
            &won_cards_by_attack,
            &self.taker.clone().unwrap().bid,
            self.bonus_petit_au_bout(),
            bonus_chelem,
            bonus_poignee,
        );

        let taker_id = self.taker.as_ref().unwrap().player.id;
        let n_defenders = (self.players.len() - 1) as f64;

        for player in &mut self.players {
            let score = if player.id == taker_id {
                attack_score * n_defenders
            } else {
                -attack_score
            };
            player.update_score(score);
        }
    }
    fn show_score(&self) {
        println!("\n--- Deal scores ---");
        for player in &self.players {
            println!("  {}: {:.1}", player.name, player.score());
        }
    }
}
impl DealGetters for Deal {
    fn bonus_petit_au_bout(&self) -> Option<Side> {
        let last_trick = self.tricks.last()?;
        if last_trick.has_petit_au_bout() {
            Some(last_trick.winner_side)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Dealing {
    Kitty,
    Player,
}
impl Distribution<Dealing> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dealing {
        match rng.gen_bool(0.4) {
            true => Dealing::Kitty,
            false => Dealing::Player,
        }
    }
}

fn get_deal_size(dealing: &Dealing) -> usize {
    match dealing {
        Dealing::Kitty => DEAL_SIZE_KITTY,
        Dealing::Player => DEAL_SIZE_PLAYERS,
    }
}
fn clear_hand(players: &mut Vec<Player>) {
    for player in players {
        player.hand = Hand::default();
    }
}

fn draw_kitty_or_player(kitty: &[Card], kitty_expected_size: usize) -> Dealing {
    if kitty.len() < kitty_expected_size {
        rand::random()
    } else {
        Dealing::Player
    }
}

fn draw_cards(deck: &[Card], players: &mut Vec<Player>, kitty: &mut Kitty) {
    let mut index: usize = 0;
    let mut dealing = Dealing::Player;
    let mut player_index = 0;

    clear_hand(players);
    while index < deck.len() {
        // Guard: if a player deal would overshoot the deck, force a kitty deal.
        // This can only happen when the kitty still needs cards (remaining == kitty_needed),
        // because once the kitty is full the remaining count is always a multiple of 3.
        let remaining = deck.len() - index;
        if dealing == Dealing::Player && remaining < DEAL_SIZE_PLAYERS {
            dealing = Dealing::Kitty;
        }
        let end_of_range = index + get_deal_size(&dealing);
        let split = &deck[index..end_of_range];
        match dealing {
            Dealing::Kitty => {
                kitty.cards.extend(split.to_vec());
            }
            Dealing::Player => {
                players[player_index].hand.cards.extend(split.to_vec());
                player_index = get_next_index(players, player_index);
            }
        }
        index = end_of_range;
        dealing = draw_kitty_or_player(&kitty.cards, kitty.max_size);
    }
}

fn collect_bids(players: &Vec<Player>, mut taker: Option<Taker>, bid: &mut Bid) -> Option<Taker> {
    if players.len() <= 1 {
        return taker;
    }

    let mut takers: Vec<Player> = Vec::new();
    for player in players {
        let new_bid = player.bid(bid);

        if new_bid != Bids::Pass {
            taker = Some(Taker {
                player: player.clone(),
                bid: new_bid,
            });
            takers.push(player.clone());
        }
        println!("{} makes the following bid: {}", player.name, new_bid);
    }

    if takers.len() > 1 {
        collect_bids(&takers, taker, bid)
    } else {
        taker.clone()
    }
}

fn merge_won_cards(players: &[Player]) -> Vec<Card> {
    let mut cards = Vec::new();
    for player in players {
        if player.hand.side == Side::Attack {
            cards.extend(player.hand.won_cards.clone());
        }
    }
    cards
}

fn compute_chelem_result(tricks: &[Trick], players: &[Player]) -> Option<Chelem> {
    let all_won_by_attack =
        !tricks.is_empty() && tricks.iter().all(|t| t.winner_side == Side::Attack);

    let announced = players.iter().any(|p| {
        p.hand
            .bonus_chelem
            .as_ref()
            .map(|c| c.state == ChelemState::Announced)
            .unwrap_or(false)
    });

    match (announced, all_won_by_attack) {
        (true, true) => Some(Chelem {
            state: ChelemState::Announced,
            result: Some(ChelemResult::AnnouncedAndSucceed),
        }),
        (true, false) => Some(Chelem {
            state: ChelemState::Announced,
            result: Some(ChelemResult::AnnouncedAndLost),
        }),
        (false, true) => Some(Chelem {
            state: ChelemState::NotAnnounced,
            result: Some(ChelemResult::NotAnnouncedAndSucceed),
        }),
        (false, false) => None,
    }
}

fn best_poignee(players: &[Player]) -> Option<Poignee> {
    players.iter().fold(None, |best, player| {
        let p = player.hand.bonus_poignee.clone();
        match (&best, &p) {
            (_, None) => best,
            (None, Some(_)) => p,
            (Some(Poignee::Triple), _) => best,
            (Some(Poignee::Double), Some(Poignee::Triple)) => p,
            (Some(Poignee::Simple), Some(Poignee::Double | Poignee::Triple)) => p,
            _ => best,
        }
    })
}
