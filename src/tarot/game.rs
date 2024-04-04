const PLAYER_NAMES: [&str; 5] = ["A", "B", "C", "D", "E"];

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
}
impl Game {
    pub fn new(n_players: u8) -> Self {
        let mut players: Vec<Player> = vec![];
        for i in 0..n_players {
            players.push(Player::new(PLAYER_NAMES[i as usize]));
        }
        Game { players }
    }
    pub fn reorder_players(&mut self, dealer: Option<u8>) {
        let next_player_index = (dealer.unwrap() + 1) % self.players.len() as u8;
        let first_players = &self.players[next_player_index as usize..];
        let second_players = &self.players[..next_player_index as usize];
        self.players = [first_players, second_players].concat();
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
        }
    }
}
