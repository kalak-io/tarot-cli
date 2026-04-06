use rand::Rng;
use std::str::FromStr;

use super::card::{Card, CardGetters};

pub fn random_int_in_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn get_next_index<T>(vector: &[T], current_index: usize) -> usize {
    if current_index < vector.len() - 1 {
        current_index + 1
    } else {
        0
    }
}

pub fn display<T: std::fmt::Display>(vector: &[T]) {
    for vect in vector {
        print!("{}", vect);
    }
    println!();
}

pub fn compare<T>(a: &T, b: Option<&T>, comparator: fn(&T, &T) -> bool) -> bool {
    match b {
        Some(b) => comparator(a, b),
        None => true,
    }
}

pub fn reorder<T: Clone>(series: &[T], index: usize) -> Vec<T> {
    let start = &series[index..];
    let end = &series[..index];
    [start, end].concat()
}

fn display_enumeration<T: std::fmt::Display>(vector: &[T]) {
    for (index, vect) in vector.iter().enumerate() {
        print!("{}. {}\t", index, vect);
    }
    println!();
}

fn prompt_selection() -> Result<usize, <usize as FromStr>::Err> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<usize>()
}

pub fn select<T: std::fmt::Display + std::marker::Copy>(
    message: Option<&str>,
    from: Option<Vec<T>>,
) -> Option<T> {
    if let Some(message) = message {
        println!("\n{}", message);
    }

    match from {
        Some(from) => {
            println!("Select an option between 0 and {}", from.len() - 1);
            display_enumeration(&from);
            match prompt_selection() {
                Ok(index) => {
                    if index < from.len() {
                        Some(from[index])
                    } else {
                        println!(
                            "Invalid input. Please enter a number lower or equal than {}",
                            from.len() - 1
                        );
                        select(message, Some(from))
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    select(message, Some(from))
                }
            }
        }
        None => {
            println!("\nNo options available");
            None
        }
    }
}

pub fn subtract(a: &mut Vec<Card>, b: &[Card]) {
    a.retain(|x| !b.contains(x));
}

pub fn card_box_lines(cards: &[Card]) -> Vec<String> {
    let mut lines = Vec::new();
    for chunk in cards.chunks(9) {
        let mut top = String::new();
        let mut rank_row = String::new();
        let mut suit_row = String::new();
        let mut bot = String::new();
        for (i, card) in chunk.iter().enumerate() {
            if i > 0 {
                top.push(' ');
                rank_row.push(' ');
                suit_row.push(' ');
                bot.push(' ');
            }
            if card.is_oudler() {
                top.push_str("╔═══╗");
                bot.push_str("╚═══╝");
            } else {
                top.push_str("┌───┐");
                bot.push_str("└───┘");
            }
            rank_row.push_str(&format!("│{}│", card_rank_label(card)));
            suit_row.push_str(&format!("│ {} │", card.suit.icon));
        }
        lines.push(top);
        lines.push(rank_row);
        lines.push(suit_row);
        lines.push(bot);
    }
    lines
}

pub fn display_cards(cards: &[Card]) {
    for line in card_box_lines(cards) {
        println!("{}", line);
    }
}

pub fn card_rank_label(card: &Card) -> String {
    match card.name().as_str() {
        "Fool" => "Foo".to_string(),
        "King" => "Kng".to_string(),
        "Queen" => "Que".to_string(),
        "Knight" => "Knt".to_string(),
        "Jack" => "Jck".to_string(),
        _ => {
            if card.rank < 10 {
                format!(" {} ", card.rank)
            } else {
                format!("{} ", card.rank)
            }
        }
    }
}
