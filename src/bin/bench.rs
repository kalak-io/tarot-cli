// Plays many all-bot deals with a fixed seed and prints statistics on the luck of the deal.
// The game prints every deal on stdout, so the report goes to stderr:
//     cargo run --release --bin bench -- [deals per player count] [seed] > /dev/null
use std::time::Instant;

use tarot_cli::common::{
    bid::{taker_evaluation, Bids},
    deal::{Deal, DealActions, DealGetters},
    game::{Game, GameActions, ReorderBy, MAX_PLAYERS, MIN_PLAYERS},
    hand::Side,
    score::{compute_oudlers, diff_points},
};

const DEFAULT_DEALS: usize = 10_000;
const DEFAULT_SEED: u64 = 42;
const BIDS: [Bids; 5] = [
    Bids::Pass,
    Bids::Take,
    Bids::Guard,
    Bids::GuardWithout,
    Bids::GuardAgainst,
];

fn bid_index(bid: Bids) -> usize {
    BIDS.iter().position(|b| *b == bid).unwrap()
}

#[derive(Default)]
struct Stats {
    deals: usize,
    redeals: usize,
    // Bid that each dealt hand is worth for a bot, by index in BIDS
    hand_bids: [usize; 5],
    taker_bids: [usize; 5],
    won_by_bid: [usize; 5],
    margin_by_bid: [f64; 5],
    // By number of oudlers in the taker's hand at bid time
    taker_oudlers: [usize; 4],
    won_by_oudlers: [usize; 4],
    petit_au_bout: usize,
    chelem: usize,
    poignees: usize,
    // By seat: 0 bids first, the last seat is the dealer
    seat_taker: Vec<usize>,
    seat_score: Vec<f64>,
    taker_scores: Vec<f64>,
    not_zero_sum: usize,
}

// Plays `n_deals` deals the way src/main.rs does, skipping deals where everyone passes
fn run(n_players: u8, n_deals: usize, seed: u64) -> Stats {
    let n = n_players as usize;
    let mut stats = Stats {
        seat_taker: vec![0; n],
        seat_score: vec![0.0; n],
        ..Default::default()
    };
    let mut game = Game::new_bots(n_players, seed);
    while stats.deals < n_deals {
        game.split_deck();
        game.update_dealer();
        game.reorder_players(ReorderBy::Dealer);
        let mut deal = Deal::new(&mut game.players, &mut game.deck, &mut game.rng);
        let seats: Vec<u8> = deal.players.iter().map(|p| p.id).collect();
        for player in &deal.players {
            stats.hand_bids[bid_index(taker_evaluation(&player.hand.cards, n))] += 1;
        }

        deal.take_bids();
        let Some(taker) = deal.taker.as_ref() else {
            stats.redeals += 1;
            continue;
        };
        let (taker_id, bid) = (taker.player.id, taker.bid);
        let taker_seat = seats.iter().position(|id| *id == taker_id).unwrap();
        let oudlers = compute_oudlers(&deal.players[taker_seat].hand.cards);

        play_deal(&mut deal);

        let margin = diff_points(&deal.attack_cards());
        let won = margin >= 0.0;
        let bid = bid_index(bid);
        stats.deals += 1;
        stats.taker_bids[bid] += 1;
        stats.won_by_bid[bid] += won as usize;
        stats.margin_by_bid[bid] += margin;
        stats.taker_oudlers[oudlers] += 1;
        stats.won_by_oudlers[oudlers] += won as usize;
        stats.seat_taker[taker_seat] += 1;
        stats.petit_au_bout += deal.bonus_petit_au_bout().is_some() as usize;
        stats.chelem += deal.tricks.iter().all(|t| t.winner_side == Side::Attack) as usize;
        stats.poignees += deal
            .players
            .iter()
            .filter(|p| p.hand.bonus_poignee.is_some())
            .count();

        // Deal players start from the game totals, so the difference is this deal's score
        let mut total = 0.0;
        for (seat, id) in seats.iter().enumerate() {
            let after = deal.players.iter().find(|p| p.id == *id).unwrap().score();
            let before = game.players.iter().find(|p| p.id == *id).unwrap().score();
            stats.seat_score[seat] += after - before;
            total += after - before;
            if *id == taker_id {
                stats.taker_scores.push(after - before);
            }
        }
        stats.not_zero_sum += (total.abs() > 1e-9) as usize;

        game.update_scores(&deal.players);
        game.collect_deck(&deal.players, &deal.kitty.cards);
    }
    stats
}

// The steps of src/main.rs after the bids
fn play_deal(deal: &mut Deal) {
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
}

fn percent(part: usize, total: usize) -> String {
    if total == 0 {
        "-".to_string()
    } else {
        format!("{:.2}%", 100.0 * part as f64 / total as f64)
    }
}

fn report(n_players: u8, seed: u64, seconds: f64, stats: &Stats) -> String {
    let n = n_players as usize;
    let names = ["Pass", "Take", "Guard", "GuardWithout", "GuardAgainst"];
    let hands: usize = stats.hand_bids.iter().sum();
    let mut lines = vec![
        format!(
            "\n=== {n_players} players: {} deals played, seed {seed}, {seconds:.1}s ===",
            stats.deals
        ),
        format!(
            "Redeals (everyone passed): {} ({} of deals dealt)",
            stats.redeals,
            percent(stats.redeals, stats.redeals + stats.deals)
        ),
        "Bid that each dealt hand is worth for a bot:".to_string(),
    ];
    for (name, count) in names.iter().zip(stats.hand_bids) {
        lines.push(format!("  {name:<13} {}", percent(count, hands)));
    }
    lines.push("Taker bid | share of deals | contract won | average margin".to_string());
    for (i, name) in names.iter().enumerate().skip(1) {
        let count = stats.taker_bids[i];
        let margin = if count > 0 {
            stats.margin_by_bid[i] / count as f64
        } else {
            0.0
        };
        lines.push(format!(
            "  {name:<13} {:>8} {:>10} {margin:>8.1}   (n={count})",
            percent(count, stats.deals),
            percent(stats.won_by_bid[i], count)
        ));
    }
    let won: usize = stats.won_by_bid.iter().sum();
    lines.push(format!(
        "  {:<13} {:>8} {:>10}",
        "All",
        "100%",
        percent(won, stats.deals)
    ));
    lines.push("Oudlers in the taker's hand at bid time | share | contract won".to_string());
    for oudlers in 0..4 {
        lines.push(format!(
            "  {oudlers} oudlers  {:>8} {:>10}",
            percent(stats.taker_oudlers[oudlers], stats.deals),
            percent(stats.won_by_oudlers[oudlers], stats.taker_oudlers[oudlers])
        ));
    }
    lines.push(format!(
        "Petit au bout: {}   Chelem: {}   Poignees declared per deal: {:.3}",
        percent(stats.petit_au_bout, stats.deals),
        percent(stats.chelem, stats.deals),
        stats.poignees as f64 / stats.deals as f64
    ));
    lines.push("Seat (0 bids first) | taker share | average score per deal".to_string());
    for seat in 0..n {
        let label = if seat == n - 1 {
            format!("{seat} (dealer)")
        } else {
            seat.to_string()
        };
        lines.push(format!(
            "  {label:<12} {:>8} {:>9.2}",
            percent(stats.seat_taker[seat], stats.deals),
            stats.seat_score[seat] / stats.deals as f64
        ));
    }
    let mut scores = stats.taker_scores.clone();
    scores.sort_by(f64::total_cmp);
    let quantile = |q: f64| scores[((scores.len() - 1) as f64 * q) as usize];
    lines.push(format!(
        "Taker score per deal: min {:.0}, p10 {:.0}, median {:.0}, p90 {:.0}, max {:.0}",
        quantile(0.0),
        quantile(0.1),
        quantile(0.5),
        quantile(0.9),
        quantile(1.0)
    ));
    lines.push(format!(
        "Deals whose scores do not sum to 0: {}",
        stats.not_zero_sum
    ));
    lines.join("\n")
}

fn parse_arg<T: std::str::FromStr>(args: &[String], index: usize, default: T) -> T {
    match args.get(index) {
        None => default,
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("Invalid argument: {arg}");
            eprintln!("Usage: bench [deals per player count] [seed]");
            std::process::exit(2);
        }),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_deals = parse_arg(&args, 1, DEFAULT_DEALS);
    let seed = parse_arg(&args, 2, DEFAULT_SEED);
    if n_deals == 0 {
        eprintln!("The number of deals must be at least 1");
        std::process::exit(2);
    }
    for n_players in MIN_PLAYERS..=MAX_PLAYERS {
        let start = Instant::now();
        let stats = run(n_players, n_deals, seed);
        let seconds = start.elapsed().as_secs_f64();
        eprintln!("{}", report(n_players, seed, seconds, &stats));
    }
}
