// use std::env;
// use std::process;

use common::bid::{collect_bid, Bid};
use common::deal::Deal;
use common::game::{collect_deck, Game};
use common::trick::{self, Trick};
use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    // run(config);

    let mut game = Game::default(); // use new() after prompt config from user

    // println!("{} players", game.players.len());
    // println!("{:?}", game.players);

    // println!("{} deck", game.deck.len());
    // println!("{} deals", game.deals.len());

    // println!("{:?}", game.deck);

    let mut deals = Vec::new();
    loop {
        println!("{} cards in deck", game.deck.len());
        let mut deal = Deal::new(&mut game.players, &mut game.deck);
        // println!("{:?}", deal);

        // TODO: move collect_bid in Deal struct
        deal.taker = collect_bid(&deal.players);
        if deal.taker.bid == Bid::Passe {
            println!("Nobody made a bid. Starting a new deal...");
            game.deck = collect_deck(&deal.players);
            continue;
        }
        deal.show_taker();
        // TODO: call king in tarot at 5
        deal.show_kitty();

        let trick = Trick::new();

        deal.tricks.push(trick);
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
