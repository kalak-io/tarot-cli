// use std::env;
// use std::process;

use common::bid::Bid;
use common::deal::Deal;
use common::game::Game;
use common::trick::Trick;
use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    let mut game = Game::default(); // TODO: use new() after prompt config from user
    let mut deals = Vec::new();

    loop {
        // Start of turn
        game.split_deck();
        game.update_dealer();
        game.reorder_players();

        let mut deal = Deal::new(&mut game.players, &mut game.deck);

        deal.collect_bids();
        if deal.taker.bid == Bid::Passe {
            println!("Nobody made a bid. Starting a new deal...");
            game.collect_deck(&deal.players);
            continue;
        }
        deal.show_taker();
        deal.call_king();
        deal.show_kitty();
        deal.play_tricks();

        deal.compute_score();
        deal.show_score();

        // End of turn
        game.collect_deck(&deal.players);
        deals.push(deal);
        break;
    }

    println!("\n\nThanks for playing !");
}
