use common::deal::{Deal, DealActions};
use common::game::{Game, GameActions, ReorderBy};
use common::utils::ask_yes_no;
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

        for player in &mut deal.players {
            player.hand.sort_by_suit();
        }

        deal.call_king();
        deal.set_side();
        deal.compose_kitty();

        for player in &mut deal.players {
            player.hand.sort_by_suit();
        }

        deal.take_chelem();
        deal.play_tricks();

        deal.set_score();
        deal.show_score();
        game.update_scores(&deal.players);

        game.collect_deck(&deal.players, &deal.kitty.cards);
        deals.push(deal);

        println!();
        if !ask_yes_no("Play another deal?") {
            break;
        }
    }

    println!("\n\nThanks for playing!");
}
