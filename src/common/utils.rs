use rand::Rng;

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

pub fn select<T: std::fmt::Display>(vector: &[T]) -> usize {
    // Display the vector with an id for each element
    // Prompt the user to select an element by id
    // Return the index of the selected element

    todo!()
}
