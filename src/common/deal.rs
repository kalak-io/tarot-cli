use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::common::{
    bid::compare_bids,
    utils::{compare, display},
};

use super::{
    bid::Bid,
    card::Card,
    player::Player,
    taker::Taker,
    trick::Trick,
    utils::{get_next_index, reorder},
};

const DEAL_SIZE_PLAYERS: usize = 3;
const DEAL_SIZE_KITTY: usize = 1;

#[derive(Debug)]
pub struct Deal {
    pub kitty: Vec<Card>,
    pub players: Vec<Player>,
    pub taker: Taker,
    pub tricks: Vec<Trick>,
    pub called_king: Option<Card>,
}
impl Deal {
    pub fn new(players: &mut Vec<Player>, deck: &mut Vec<Card>) -> Self {
        let mut kitty = Vec::new();
        draw_cards(&deck, players, &mut kitty);

        Deal {
            kitty,
            players: players.to_vec(), // TODO: is it necessary ?
            taker: Taker::default(),
            tricks: Vec::new(),
            called_king: None,
        }
    }
    pub fn update_taker(&mut self) {
        self.taker = collect_bids(&self.players, &mut self.taker);
        println!(
            "The taker is {} with a bid of {:?}",
            self.taker.player.name, self.taker.bid
        );
    }
    pub fn compose_kitty(&mut self) {
        self.kitty = match self.taker.bid {
            Bid::GardeSans | Bid::GardeContre => {
                println!("\n\nThe kitty stays hidden"); // TODO: move kitty in right place
                self.kitty.clone()
            }
            _ => {
                println!("\n\nThe kitty contains: ");
                display(&self.kitty);
                self.taker.player.compose_kitty(&self.kitty)
            }
        }
    }
    pub fn call_king(&mut self) {
        // TODO: implement logic
        if self.players.len() > 4 {
            match self.taker.player.is_human {
                true => {}
                false => {}
            }
        }
    }
    pub fn play_tricks(&mut self) {
        if self.players[0].hand.cards.len() == 0 {
            return;
        }
        let trick = Trick::default();
        for player in self.players.clone() {
            player.play(&trick);
        }
        let winner_index = trick.get_best_played_card_index();
        self.players = reorder(&self.players, winner_index);
        self.tricks.push(trick);
        self.play_tricks()
    }
    pub fn compute_score(&self) {}
    pub fn show_score(&self) {}
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

pub fn get_kitty_expected_size(n_players: usize) -> usize {
    match n_players {
        2..=4 => 6,
        5.. => 3,
        _ => 0, // maybe raise an error
    }
}

fn draw_kitty_or_player(kitty: &[Card], kitty_expected_size: usize) -> Dealing {
    match kitty.len() == kitty_expected_size {
        false => {
            let random: Dealing = rand::random();
            random
        }
        true => Dealing::Player,
    }
}

fn draw_cards(deck: &[Card], players: &mut Vec<Player>, kitty: &mut Vec<Card>) {
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
                players[player_index].hand.cards.extend(split.to_vec());
                player_index = get_next_index(&players, player_index);
            }
        }
        index = end_of_range;
        dealing = draw_kitty_or_player(&kitty, kitty_expected_size);
    }
}

fn collect_bids(players: &Vec<Player>, current_taker: &mut Taker) -> Taker {
    if players.len() <= 1 {
        return current_taker.clone();
    }

    let mut takers: Vec<Player> = Vec::new();
    for player in players {
        let bid = player.bid(current_taker);

        if bid != Bid::Passe {
            if compare(&bid, Some(&current_taker.bid), compare_bids) {
                *current_taker = Taker {
                    player: player.clone(),
                    bid,
                };
                takers.push(player.clone());
            }
        }
        println!("{} makes the following bid: {:?}", player.name, bid);
    }

    if takers.len() > 1 {
        collect_bids(&takers, current_taker)
    } else {
        current_taker.clone()
    }
}
