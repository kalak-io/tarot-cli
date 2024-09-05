use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::common::utils::display;

use super::{
    bid::{Bid, Bids},
    card::Card,
    kitty::Kitty,
    player::{Player, PlayerActions},
    taker::Taker,
    trick::Trick,
    utils::{get_next_index, reorder},
};

const DEAL_SIZE_PLAYERS: usize = 3;
const DEAL_SIZE_KITTY: usize = 1;

pub trait DealActions {
    fn take_bids(&mut self);
    fn call_king(&mut self);
    fn compose_kitty(&mut self);
    fn play_tricks(&mut self);
    fn compute_score(&self);
    fn show_score(&self);
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
    pub fn new(players: &mut Vec<Player>, deck: &mut Vec<Card>) -> Self {
        let mut kitty = Kitty::new(players.len());
        draw_cards(&deck, players, &mut kitty);

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
        // self.taker = collect_bids(&self.players, self.taker.clone(), &mut bid);
        self.taker = collect_bids(&self.players, self.taker.clone(), &mut bid);
    }
    fn call_king(&mut self) {
        if self.players.len() > 4 {
            self.called_king = Some(self.taker.clone().unwrap().player.call_king());
            println!("\nThe called king is {}", self.called_king.clone().unwrap());
        }
    }
    fn compose_kitty(&mut self) {
        match self.taker.clone().unwrap().bid {
            Bids::GardeSans | Bids::GardeContre => {
                println!("\n\nThe kitty stays hidden"); // TODO: move kitty in right place
            }
            _ => {
                println!("\n\nThe kitty contains: ");
                display(&self.kitty.cards);
                self.kitty.cards = self
                    .taker
                    .clone()
                    .unwrap()
                    .player
                    .compose_kitty(&mut self.kitty)
            }
        }
    }
    fn play_tricks(&mut self) {
        if self.players[0].hand.cards.len() == 0 {
            return;
        }
        let trick = Trick::default();
        for player in self.players.clone() {
            player.play(&trick);
        }
        let winner_index = 0; // trick.get_best_played_card_index();
        self.players = reorder(&self.players, winner_index);
        self.tricks.push(trick);
        self.play_tricks()
    }
    fn compute_score(&self) {}
    fn show_score(&self) {}
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
fn clear_cards(players: &mut Vec<Player>) {
    for player in players {
        player.hand.cards.clear();
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

    clear_cards(players);
    while index < deck.len() {
        let end_of_range = index + get_deal_size(&dealing);
        let split = &deck[index..end_of_range];
        match dealing {
            Dealing::Kitty => {
                kitty.cards.extend(split.to_vec());
            }
            Dealing::Player => {
                players[player_index].hand.cards.extend(split.to_vec());
                player_index = get_next_index(&players, player_index);
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

        if new_bid != Bids::Passe {
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
