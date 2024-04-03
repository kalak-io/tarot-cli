use std::env;

use tarot_cli::*;

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
    game.reorder_players();
    //println!("{:?}", game);

//        loop {
//    println!("{:?}", game.deck);
//    println!("SPLIT THE DECK----------");
    game.deck.split();
//    println!("{:?}", game.deck);

            // DEAL the cards between players
            // - split
            // - 3 by 3
            // - fill the kitty / dog
            // BID
            // First player (after the dealer) makes bid
            // - compute score in hands if under X points PASS
            // - else if PETITE ...
            // while all players make their bid
            // - prompt bid for the player A
            // PLAY
            // Each player plays one card
            // - check which wins
            // MARK POINT
            // - compute score of taker
            // - check if taker wins
            // - store points
//        }
}
