use super::{card::Card, chelem::Chelem};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Side {
    Attack,
    #[default]
    Defense,
}
#[derive(Debug, Clone)]
pub enum Poignee {
    Simple,
    Double,
    Triple,
}

pub trait HandActions {
    fn human_declare_poignee(&mut self);
    fn human_declare_chelem(&mut self);
    fn bot_declare_poignee(&mut self);
    fn bot_declare_chelem(&mut self);
}
#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub won_cards: Vec<Card>,
    pub side: Side,
    pub bonus_poignee: Option<Poignee>,
    pub bonus_chelem: Option<Chelem>,
}
impl Hand {
    pub fn set_side_with_called_king(&mut self, called_king: Option<Card>) {
        if let Some(called_king) = called_king {
            if self.cards.contains(&called_king) {
                self.side = Side::Attack;
            }
        }
    }
}

impl HandActions for Hand {
    fn human_declare_poignee(&mut self) {
        if self.bonus_poignee.is_some() {
            return;
        }
        let mut input = String::new();
        loop {
            println!("Do you want to declare a poignee? (yes/no)");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_lowercase();
            match input.as_str() {
                "yes" | "y" => {
                    // Ask the player to choose a poignee
                    let mut input = String::new();
                    loop {
                        println!("Choose a poignee: simple, double or triple");
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        let input = input.trim().to_lowercase();
                        match input.as_str() {
                            "simple" => {
                                self.bonus_poignee = Some(Poignee::Simple);
                                break;
                            }
                            "double" => {
                                self.bonus_poignee = Some(Poignee::Double);
                                break;
                            }
                            "triple" => {
                                self.bonus_poignee = Some(Poignee::Triple);
                                break;
                            }
                            _ => continue,
                        }
                    }
                    break;
                }
                "no" | "n" => break,
                _ => continue,
            }
        }
    }
    fn human_declare_chelem(&mut self) {
        todo!()
    }
    fn bot_declare_poignee(&mut self) {
        todo!()
    }
    fn bot_declare_chelem(&mut self) {
        todo!()
    }
}
