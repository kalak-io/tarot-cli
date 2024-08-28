// use std::env;
// use std::process;

use common::bid::Bids;
use common::deal::Deal;
use common::game::Game;
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
        println!("{:?}", game.deck);

        let mut deal = Deal::new(&mut game.players, &mut game.deck);

        deal.update_taker();
        if deal.taker.bid == Bids::Passe {
            println!("Nobody made a bid. Starting a new deal...");
            game.collect_deck(&deal.players);
            continue;
        }
        deal.call_king();
        deal.compose_kitty();
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
