use rand::Rng;

use super::{
    bid::Bids,
    card::Card,
    score::{compute_oudlers, compute_points},
};

pub fn random_int_in_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn get_next_index<T>(vector: &Vec<T>, current_index: usize) -> usize {
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

pub fn reorder<T: Clone>(serie: &Vec<T>, index: usize) -> Vec<T> {
    let start = &serie[index..];
    let end = &serie[..index];
    [start, end].concat()
}

fn display_enumeration<T: std::fmt::Display>(vector: &[T]) {
    for (index, vect) in vector.iter().enumerate() {
        print!("{}. {}\t", index, vect);
    }
    println!("");
}

pub fn prompt_selection<T: std::fmt::Display>(message: &str, data: Option<Vec<T>>) -> usize {
    if let Some(data) = data {
        println!(
            "\n{message}\nSelect an option between 0 and {}",
            data.len() - 1
        );
        display_enumeration(&data);
    } else {
        println!("{message} No options available");
        return 0; // or handle this case in some other way
    }
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
    // TODO: check that input is a number, is a valid index and is not out of bounds
}

pub fn subtract(a: &mut Vec<Card>, b: &Vec<Card>) {
    a.retain(|x| !b.contains(x));
}
