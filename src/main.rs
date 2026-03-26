use common::deal::{Deal, DealActions};
use common::game::{Game, GameActions, ReorderBy};
use tarot_cli::*;

fn main() {
    println!("Let's play Tarot!");

    let mut game = Game::default();
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

        deal.call_king();
        deal.set_side();
        deal.compose_kitty();
        deal.take_chelem();

        game.reorder_players(ReorderBy::Chelem);
        deal.play_tricks();

        deal.set_score();
        deal.show_score();

        game.collect_deck(&deal.players);
        deals.push(deal);

        println!("\nPlay another deal? (yes/no)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if !matches!(input.trim().to_lowercase().as_str(), "yes" | "y") {
            break;
        }
    }

    println!("\n\nThanks for playing!");
}
