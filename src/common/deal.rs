use rand::seq::index::sample;

use crate::common::utils::display_cards;

use super::{
    bid::{Bid, Bids},
    card::{count_cards_by_hand, Card, CardGetters},
    chelem::{Chelem, ChelemResult, ChelemState},
    hand::{Hand, Poignee, Side},
    kitty::Kitty,
    player::{Player, PlayerActions},
    score::compute_score,
    taker::Taker,
    trick::{Trick, TrickActions, TrickGetters},
    utils::{get_next_index, reorder},
};

// Cards dealt to a player at a time: 4 by 4 with 3 players, 3 by 3 otherwise
fn packet_size(n_players: usize) -> usize {
    if n_players == 3 {
        4
    } else {
        3
    }
}

pub trait DealActions {
    fn take_bids(&mut self);
    fn take_chelem(&mut self);
    fn call_king(&mut self);
    fn compose_kitty(&mut self);
    fn play_tricks(&mut self);
    fn set_side(&mut self);
    fn set_score(&mut self);
    fn show_score(&self);
}

pub trait DealGetters {
    fn bonus_petit_au_bout(&self) -> Option<Side>;
}
#[derive(Debug, Default)]
pub struct Deal {
    pub kitty: Kitty,
    pub players: Vec<Player>,
    pub taker: Option<Taker>,
    pub tricks: Vec<Trick>,
    pub called_card: Option<Card>,
    // Side that owes a low card for keeping the Fool, until it wins one
    pub fool_debt: Option<Side>,
}
impl Deal {
    pub fn new(players: &mut Vec<Player>, deck: &mut [Card]) -> Self {
        let mut kitty = Kitty::new(players.len());
        draw_cards(deck, players, &mut kitty);

        Deal {
            players: players.to_vec(), // TODO: is it necessary ?
            kitty,
            ..Default::default()
        }
    }
    fn count_side(&self, side: Side) -> usize {
        self.players.iter().filter(|p| p.hand.side == side).count()
    }
    // Index of the taker in `self.players`; `self.taker.player` is a copy made at bid time
    fn taker_index(&self) -> Option<usize> {
        let taker_id = self.taker.as_ref()?.player.id;
        self.players.iter().position(|p| p.id == taker_id)
    }
}
impl DealActions for Deal {
    fn take_bids(&mut self) {
        let mut bid = Bid::default();
        self.taker = collect_bids(&self.players, &mut bid);
    }
    fn take_chelem(&mut self) {
        if let Some(index) = self.taker_index() {
            self.players[index].declare_chelem();
        }
        // The player who announces a chelem leads the first trick
        if let Some(index) = find_announced_chelem(&self.players) {
            self.players = reorder(&self.players, index);
        }
    }
    fn call_king(&mut self) {
        if self.players.len() > 4 {
            self.called_card = Some(self.taker.clone().unwrap().player.call_king());
            println!("\nThe called card is {}", self.called_card.unwrap());
        }
    }
    fn compose_kitty(&mut self) {
        let bid = self.taker.as_ref().unwrap().bid;
        match bid {
            Bids::GuardWithout | Bids::GuardAgainst => {
                println!("\n\nThe kitty stays hidden");
            }
            _ => {
                println!("\n\nThe kitty contains: ");
                display_cards(&self.kitty.cards);
                let taker_index = self.taker_index().unwrap();
                // Temporarily take kitty out to satisfy borrow checker
                let mut kitty = std::mem::take(&mut self.kitty);
                self.players[taker_index].compose_kitty(&mut kitty);
                self.kitty = kitty;
            }
        }
    }
    fn play_tricks(&mut self) {
        if self.players[0].hand.cards.is_empty() {
            return;
        }

        let n_players = self.players.len();
        // The first lead cannot be in the called card's suit (5-player game)
        let mut trick = Trick {
            called_card: self.called_card.filter(|_| self.tricks.is_empty()),
            ..Default::default()
        };
        for player in &mut self.players {
            if count_cards_by_hand(n_players) == player.hand.cards.len() as u8 {
                player.declare_poignee(n_players);
            }
            player.play(&mut trick);
        }
        let is_last_trick = self.players[0].hand.cards.is_empty();
        let fool_index = trick.played_cards.iter().position(|c| c.is_fool());
        let leader_side = self.players[0].hand.side;
        let winner_index = match fool_index {
            // A side that won every trick wins the last one by leading the Fool (chelem)
            Some(0)
                if is_last_trick && self.tricks.iter().all(|t| t.winner_side == leader_side) =>
            {
                0
            }
            _ => trick
                .get_best_played_card_index(trick.played_suit())
                .unwrap(),
        };
        let winner_side = self.players[winner_index].hand.side;

        let mut won_cards = trick.played_cards.clone();
        if let Some(fool_index) = fool_index {
            let fool_side = self.players[fool_index].hand.side;
            // The Fool stays with its side, which gives a low card in exchange.
            // Played to the last trick, it goes to the winner.
            if fool_side != winner_side && !is_last_trick {
                won_cards.retain(|c| !c.is_fool());
                self.players[fool_index]
                    .hand
                    .won_cards
                    .push(trick.played_cards[fool_index]);
                self.fool_debt = Some(fool_side);
            }
        }
        self.players[winner_index].hand.won_cards.extend(won_cards);
        if let Some(side) = self.fool_debt {
            if give_low_card(&mut self.players, side) {
                self.fool_debt = None;
            }
        }

        trick.winner_side = winner_side;
        self.players = reorder(&self.players, winner_index);
        self.tricks.push(trick);
        self.play_tricks()
    }
    fn set_side(&mut self) {
        // Set the called card holder's side (5-player game)
        if self.called_card.is_some() {
            for player in &mut self.players {
                player.hand.set_side_with_called_card(self.called_card);
            }
        }
        if let Some(index) = self.taker_index() {
            self.players[index].hand.side = Side::Attack;
        }
    }
    fn set_score(&mut self) {
        let taker = self.taker.as_ref().unwrap();
        let (taker_id, bid) = (taker.player.id, taker.bid);

        let mut won_cards_by_attack = merge_won_cards(&self.players);
        // The kitty, or the taker's discard, counts for the attack except on a Guard Against
        if bid != Bids::GuardAgainst {
            won_cards_by_attack.extend_from_slice(&self.kitty.cards);
        }
        let bonus_chelem = compute_chelem_result(&self.tricks, &self.players);
        let bonus_poignee = best_poignee(&self.players);
        let attack_score = compute_score(
            &won_cards_by_attack,
            &bid,
            self.bonus_petit_au_bout(),
            bonus_chelem,
            bonus_poignee,
        );

        // Each defender pays or gets `attack_score`. A partner (5 players) takes one share,
        // and the taker takes the rest, so the scores sum to 0.
        let n_defenders = self.count_side(Side::Defense) as f64;
        let n_partners = (self.count_side(Side::Attack) - 1) as f64;

        for player in &mut self.players {
            let score = if player.id == taker_id {
                attack_score * (n_defenders - n_partners)
            } else if player.hand.side == Side::Attack {
                attack_score
            } else {
                -attack_score
            };
            player.update_score(score);
        }
    }
    fn show_score(&self) {
        println!("\n--- Total scores ---");
        for player in &self.players {
            println!("  {}: {:.1}", player.name, player.score());
        }
    }
}
impl DealGetters for Deal {
    fn bonus_petit_au_bout(&self) -> Option<Side> {
        let last_trick = self.tricks.last()?;
        if last_trick.has_petit_au_bout() {
            Some(last_trick.winner_side)
        } else {
            None
        }
    }
}

fn clear_hand(players: &mut Vec<Player>) {
    for player in players {
        player.hand = Hand::default();
    }
}

fn draw_cards(deck: &[Card], players: &mut Vec<Player>, kitty: &mut Kitty) {
    clear_hand(players);
    let packet_size = packet_size(players.len());
    let n_packets = (deck.len() - kitty.max_size) / packet_size;
    // One kitty card goes after some of the player packets, never after the last one,
    // so the first and the last cards of the deck never go to the kitty
    let kitty_after = sample(&mut rand::rng(), n_packets - 1, kitty.max_size).into_vec();

    let mut cards = deck.iter().copied();
    let mut player_index = 0;
    for packet in 0..n_packets {
        players[player_index]
            .hand
            .cards
            .extend(cards.by_ref().take(packet_size));
        player_index = get_next_index(players, player_index);
        if kitty_after.contains(&packet) {
            kitty.cards.extend(cards.next());
        }
    }
}

// Each player bids once, and each bid must beat the one before it
fn collect_bids(players: &[Player], bid: &mut Bid) -> Option<Taker> {
    let mut taker = None;
    for player in players {
        let new_bid = player.bid(bid, players.len());
        if new_bid != Bids::Pass {
            taker = Some(Taker {
                player: player.clone(),
                bid: new_bid,
            });
        }
        println!("{} makes the following bid: {}", player.name, new_bid);
    }
    taker
}

// Moves a 0.5-point card won by `from` to a player of the other side.
// Returns false when `from` has not won such a card yet.
fn give_low_card(players: &mut [Player], from: Side) -> bool {
    let Some((giver, position)) = players.iter().enumerate().find_map(|(index, player)| {
        if player.hand.side != from {
            return None;
        }
        player
            .hand
            .won_cards
            .iter()
            .position(|card| card.score() == 0.5)
            .map(|position| (index, position))
    }) else {
        return false;
    };
    let Some(receiver) = players.iter().position(|p| p.hand.side != from) else {
        return false;
    };
    let card = players[giver].hand.won_cards.remove(position);
    players[receiver].hand.won_cards.push(card);
    true
}

fn merge_won_cards(players: &[Player]) -> Vec<Card> {
    let mut cards = Vec::new();
    for player in players {
        if player.hand.side == Side::Attack {
            cards.extend(player.hand.won_cards.clone());
        }
    }
    cards
}

fn compute_chelem_result(tricks: &[Trick], players: &[Player]) -> Option<Chelem> {
    let all_won_by_attack =
        !tricks.is_empty() && tricks.iter().all(|t| t.winner_side == Side::Attack);

    let announced = find_announced_chelem(players).is_some();

    match (announced, all_won_by_attack) {
        (true, true) => Some(Chelem {
            state: ChelemState::Announced,
            result: Some(ChelemResult::AnnouncedAndSucceed),
        }),
        (true, false) => Some(Chelem {
            state: ChelemState::Announced,
            result: Some(ChelemResult::AnnouncedAndLost),
        }),
        (false, true) => Some(Chelem {
            state: ChelemState::NotAnnounced,
            result: Some(ChelemResult::NotAnnouncedAndSucceed),
        }),
        (false, false) => None,
    }
}

fn best_poignee(players: &[Player]) -> Option<Poignee> {
    players.iter().fold(None, |best, player| {
        let p = player.hand.bonus_poignee;
        match (&best, &p) {
            (_, None) => best,
            (None, Some(_)) => p,
            (Some(Poignee::Triple), _) => best,
            (Some(Poignee::Double), Some(Poignee::Triple)) => p,
            (Some(Poignee::Simple), Some(Poignee::Double | Poignee::Triple)) => p,
            _ => best,
        }
    })
}

fn find_announced_chelem(players: &[Player]) -> Option<usize> {
    players.iter().position(|player| {
        player
            .hand
            .bonus_chelem
            .as_ref()
            .is_some_and(|c| c.state == ChelemState::Announced)
    })
}
