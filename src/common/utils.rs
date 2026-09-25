use std::str::FromStr;

use super::card::{Card, CardGetters, CardSuits};

pub fn get_next_index<T>(vector: &[T], current_index: usize) -> usize {
    if current_index < vector.len() - 1 {
        current_index + 1
    } else {
        0
    }
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

// Reads one line from stdin. The game ends when stdin is closed (Ctrl-D).
pub fn read_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(0) => {
            println!("\nInput closed. Thanks for playing!");
            std::process::exit(0);
        }
        Ok(_) => input,
        Err(error) => panic!("Failed to read line: {error}"),
    }
}

// Asks until the answer is yes or no
pub fn ask_yes_no(question: &str) -> bool {
    loop {
        println!("{question} (yes/no)");
        match read_input().trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => continue,
        }
    }
}

fn prompt_selection() -> Result<usize, <usize as FromStr>::Err> {
    read_input().trim().parse::<usize>()
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

const CARDS_PER_ROW: usize = 9;

// Top border, rank, suit and bottom border rows for one row of cards
fn card_box_row(cards: &[Card]) -> [String; 4] {
    let mut rows: [Vec<String>; 4] = Default::default();
    for card in cards {
        let (top, bottom) = if card.is_oudler() {
            ("╔═══╗", "╚═══╝")
        } else {
            ("┌───┐", "└───┘")
        };
        rows[0].push(top.to_string());
        rows[1].push(format!("│{}│", card_rank_label(card)));
        rows[2].push(format!("│ {} │", card.suit.icon));
        rows[3].push(bottom.to_string());
    }
    rows.map(|row| row.join(" "))
}

pub fn card_box_lines(cards: &[Card]) -> Vec<String> {
    cards.chunks(CARDS_PER_ROW).flat_map(card_box_row).collect()
}

pub fn display_cards(cards: &[Card]) {
    for line in card_box_lines(cards) {
        println!("{}", line);
    }
}

// Same as `display_cards`, with each card's selection number under it
fn display_cards_enumerated(cards: &[Card]) {
    for (row_index, row) in cards.chunks(CARDS_PER_ROW).enumerate() {
        for line in card_box_row(row) {
            println!("{}", line);
        }
        let numbers: Vec<String> = (0..row.len())
            .map(|i| format!("{:^5}", row_index * CARDS_PER_ROW + i))
            .collect();
        println!("{}", numbers.join(" "));
    }
}

pub fn select_card(message: Option<&str>, from: Option<Vec<Card>>) -> Option<Card> {
    if let Some(message) = message {
        println!("\n{}", message);
    }
    match from {
        Some(from) => {
            println!("Select an option between 0 and {}", from.len() - 1);
            display_cards_enumerated(&from);
            match prompt_selection() {
                Ok(index) if index < from.len() => Some(from[index]),
                Ok(_) => {
                    println!(
                        "Invalid input. Please enter a number lower or equal than {}",
                        from.len() - 1
                    );
                    select_card(message, Some(from))
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    select_card(message, Some(from))
                }
            }
        }
        None => {
            println!("\nNo options available");
            None
        }
    }
}

pub fn card_rank_label(card: &Card) -> String {
    match (card.rank, card.suit.name) {
        (22, CardSuits::Trumps) => "Foo".to_string(), // Fool
        (r @ 11..=14, CardSuits::Trumps) => format!("{} ", r), // Trumps have no face cards
        (14, _) => "Kng".to_string(),                 // King
        (13, _) => "Que".to_string(),                 // Queen
        (12, _) => "Knt".to_string(),                 // Knight
        (11, _) => "Jck".to_string(),                 // Jack
        (r, _) if r < 10 => format!(" {} ", r),
        (r, _) => format!("{} ", r),
    }
}
