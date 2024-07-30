use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::common::utils::display;

use super::{
    bid::Bid, card::Card, game::split_deck, player::Player, taker::Taker, trick::Trick,
    utils::get_next_index,
};

const DEAL_SIZE_PLAYERS: usize = 3;
const DEAL_SIZE_KITTY: usize = 1;

#[derive(Debug)]
pub struct Deal {
    // dealer: Player,
    kitty: Vec<Card>,
    pub players: Vec<Player>,
    pub taker: Taker,
    pub tricks: Vec<Trick>,
}
impl Deal {
    pub fn new(players: &mut Vec<Player>, deck: &mut Vec<Card>) -> Self {
        let dealer = patch_dealer(players);
        let mut kitty = Vec::new();

        // println!("BEFORE: {:?}", kitty.len());
        reorder_players(players);
        split_deck(deck);
        deal_cards(&deck, players, &mut kitty);
        // println!("AFTER: {:?}", kitty.len());

        Deal {
            // dealer,
            kitty,
            players: players.to_vec(),
            taker: Taker::default(),
            tricks: Vec::new(),
        }
    }
    pub fn show_taker(&self) {
        println!(
            "The taker is {} with a bid of {:?}",
            self.taker.player.name, self.taker.bid
        );
    }
    pub fn show_kitty(&self) {
        match self.taker.bid {
            Bid::GardeSans | Bid::GardeContre => println!("The kitty stays hidden"),
            _ => {
                println!("The kitty contains: ");
                display(&self.kitty)
            }
        }
    }
}

fn find_dealer(players: &Vec<Player>) -> usize {
    players.iter().position(|player| player.is_dealer).unwrap()
}

pub fn toggle_is_dealer(players: &mut Vec<Player>, index: usize) {
    players[index].is_dealer ^= true;
}

fn patch_dealer(players: &mut Vec<Player>) -> Player {
    let index = find_dealer(players);
    toggle_is_dealer(players, index);

    let next_index = get_next_index(&players, index);
    toggle_is_dealer(players, next_index);

    println!("The dealer is {}", players[next_index].name);
    players[next_index].clone()
}

fn reorder_players(players: &mut Vec<Player>) {
    let dealer_index = find_dealer(players);
    let start_index = get_next_index(players, dealer_index);
    let start = &players[start_index..];
    let end = &players[..start_index];
    let new_players = [start, end].concat();
    players.clear();
    players.extend_from_slice(&new_players);
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
        player.cards.clear();
    }
}

fn get_kitty_expected_size(n_players: usize) -> usize {
    match n_players {
        2..=4 => 6,
        5.. => 3,
        _ => 0, // maybe raise an error
    }
}

fn deal_kitty_or_player(kitty: &[Card], kitty_expected_size: usize) -> Dealing {
    match kitty.len() == kitty_expected_size {
        false => {
            let random: Dealing = rand::random();
            random
        }
        true => Dealing::Player,
    }
}

fn deal_cards(deck: &[Card], players: &mut Vec<Player>, kitty: &mut Vec<Card>) {
    let mut index: usize = 0;
    let mut dealing = Dealing::Player;
    let mut player_index = 0;
    let kitty_expected_size = get_kitty_expected_size(players.len());

    clear_cards(players);
    while index < deck.len() {
        let end_of_range = index + get_deal_size(&dealing);
        let split = &deck[index..end_of_range];
        match dealing {
            Dealing::Kitty => {
                kitty.extend(split.to_vec());
            }
            Dealing::Player => {
                players[player_index].cards.extend(split.to_vec());
                player_index = get_next_index(&players, player_index);
            }
        }
        index = end_of_range;
        dealing = deal_kitty_or_player(&kitty, kitty_expected_size);
    }
}
