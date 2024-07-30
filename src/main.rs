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
        // println!("{:?}", deal);

        deal.collect_bids();
        if deal.taker.bid == Bid::Passe {
            println!("Nobody made a bid. Starting a new deal...");
            game.collect_deck(&deal.players);
            continue;
        }
        deal.show_taker();
        if game.players.len() == 5 {
            deal.call_king();
        }
        // TODO: call king in tarot at 5
        deal.show_kitty();

        let trick = Trick::new();

        deal.tricks.push(trick);

        // End of turn
        game.collect_deck(&deal.players);
        deals.push(deal);
        break;
    }

    // println!("{:?}", deals);

    println!("\n\nThanks for playing !");
}

// Game
// init players
// init cards (deck)
// init deals

// deal
// split_deck
// update dealer
// deal cards
// make tricks
// compute score

// Trick
// play cards

// Player
// make_bid
// play_card
