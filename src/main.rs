use std::env;

//use std::process::Command; // to delete

//use tarot_cli::*;

mod tarot;
use crate::tarot::game::Game;
use crate::tarot::deck::Deck;
use crate::tarot::hand::Hand;

// The user is always the player named A (you)

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg_n_players = match args.get(1) {
        Some(value) => value,
        None => {
            println!("Not enough arguments provided!");
            return;
        }
    };

    let n_players = match arg_n_players.parse::<u8>() {
        Ok(value) => value,
        Err(e) => {
            println!("Unable to parse number from argument: {}", e);
            return;
        }
    };

    match n_players {
        2 | 3 | 4 | 5 => {}
        _ => {
            println!("Invalid number of players: {}", n_players);
            return;
        }
    }

    let mut game = Game::new(n_players);
    let mut deck = Deck::new();
    deck.update_dealer(n_players);
    loop {
        game.reorder_players(deck.dealer);
        let mut hand = Hand::new(game.players);
        deck.deal(hand.players, hand.kitty);
        // hand.show();
        // hand.split();
        // hand.deal();
        // hand.show();
        // deck.show();
        deck.update_dealer(n_players);
        break;
    }
    // deck.split();
    // println!("{:?}", deck);
    // game.deck.show();
    // loop {
    //     game.reorder_players();
    //     let mut hand = Hand::new(&game.deck.cards, &game.players);
    //     hand.show();
    //     hand.split();
    //     hand.deal();
    //     hand.show();
    //
    //     let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    //     let _result = child.wait().unwrap();
    //
    //         // DEAL the cards between players
    //         // - split
    //         // - 3 by 3
    //         // - fill the kitty / dog
    //         // BID
    //         // First player (after the dealer) makes bid
    //         // - compute score in hands if under X points PASS
    //         // - else if PETITE ...
    //         // while all players make their bid
    //         // - prompt bid for the player A
    //         // PLAY
    //         // Each player plays one card
    //         // - check which wins
    //         // MARK POINT
    //         // - compute score of taker
    //         // - check if taker wins
    //         // - store points
    //     game.update_dealer();
    // }
}
