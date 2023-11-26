use proconio::input;
use std::collections::HashMap;

const DEBUG: bool = false;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut vote_count: HashMap<usize, usize> = HashMap::new();
    let mut max_score: usize = 0;
    let mut max_candidate: usize = 0;

    for i in 0..m {
        // Recording the current vote
        let current_candidate = a[i];
        let current_score;
        if !vote_count.contains_key(&current_candidate) {
            vote_count.insert(current_candidate, 1);
            current_score = 1;
        } else {
            let new_count = vote_count.get(&current_candidate).unwrap() + 1;
            vote_count.insert(current_candidate, new_count);
            current_score = new_count;
        }
        if DEBUG { println!("Cand: {}, score: {}, best:{}, best score: {}", current_candidate, current_score, max_candidate, max_score)}
        // Now let's figure out the winner for this round
        if current_score > max_score {
            max_score = current_score;
            max_candidate = current_candidate;
        } else {
            if current_score == max_score {
                max_candidate = current_candidate.min(max_candidate);
            }
        }

        println!("{}", max_candidate);
    }
}
