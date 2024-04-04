pub fn compute_score(cards: &Vec<Card>) -> f64 {
    let mut score = 0.0;
    for card in cards {
        score += card.score();
    }
    score
}

#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/compute_score.rs"]
mod compute_score;
