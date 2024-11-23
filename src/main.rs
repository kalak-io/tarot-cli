// use std::env;
// use std::process;

use common::chelem::ChelemState;
use common::deal::{Deal, DealActions};
use common::game::{Game, GameActions, ReorderBy};
use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    let mut game = Game::default(); // TODO: use new() after prompt config from user
    let mut deals = Vec::new();

    loop {
        game.split_deck();
        game.update_dealer();
        game.reorder_players(ReorderBy::Dealer);

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
        deal.take_chelem(); // TODO: the person that announces the chelem becomes the first player

        if let Some(taker) = &deal.taker {
            println!(
                "The taker is {} with a bid of {:?}",
                taker.player.name, taker.bid
            );
            println!("{:?}", taker.player.hand.bonus_chelem); //TODO toggle Chelem on player in taker doesn't work -> check clone usage
            if let Some(chelem) = &taker.player.hand.bonus_chelem {
                if chelem.state != ChelemState::NotAnnounced {
                    println!("A chelem is announced");
                }
            }
        }
        deal.call_king();
        deal.set_side();
        deal.compose_kitty();
        deal.take_chelem();

        game.reorder_players(ReorderBy::Chelem); // TODO: reorder to start with the player who announced a chelem
        deal.play_tricks();

        deal.set_score(); // TODO
        deal.show_score(); // TODO

        game.collect_deck(&deal.players);
        deals.push(deal);
        break;
    }

    println!("\n\nThanks for playing !");
}
