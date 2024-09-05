// use std::env;
// use std::process;

use common::deal::{Deal, DealActions};
use common::game::{Game, GameActions};
use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    let mut game = Game::default(); // TODO: use new() after prompt config from user
    let mut deals = Vec::new();

    loop {
        game.split_deck();
        game.update_dealer();
        game.reorder_players();

        let mut deal = Deal::new(&mut game.players, &mut game.deck);

        deal.take_bids();
        match &deal.taker {
            None => {
                println!("Nobody made a bid. Starting a new deal...");
                continue;
            }
            Some(taker) => {
                println!(
                    "The taker is {} with a bid of {:?}",
                    taker.player.name, taker.bid
                );
            }
        }
        deal.call_king(); // TODO
        deal.compose_kitty();
        deal.play_tricks();

        deal.compute_score();
        deal.show_score();

        game.collect_deck(&deal.players);
        deals.push(deal);
        break;
    }

    println!("\n\nThanks for playing !");
}
