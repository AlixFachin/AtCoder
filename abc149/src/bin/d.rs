use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn get_score(r: &usize, s: &usize, p: &usize, player: &char, opponent: &char) -> usize {
    match player {
        'r' =>
            match opponent {
                'r' => 0,
                's' => *r,
                'p' => 0,
                _ => panic!("Invalid input"),
            }
        's' =>
            match opponent {
                'r' => 0,
                's' => 0,
                'p' => *s,
                _ => panic!("Invalid input"),
            }
        'p' =>
            match opponent {
                'r' => *p,
                's' => 0,
                'p' => 0,
                _ => panic!("Invalid input"),
            }
        _ => panic!("Invalid input"),
    }
}

fn get_winning_play(opponent: &char) -> char {
    match opponent {
        'r' => 'p',
        's' => 'r',
        'p' => 's',
        _ => panic!("Invalid input"),
    }
}

fn get_other_play(play: &char) -> char {
    match play {
        'r' => 's',
        's' => 'p',
        'p' => 'r',
        _ => panic!("Invalid input"),
    }
}

fn get_neither_play(play1: &char, play2: &char) -> char {
    match (play1, play2) {
        ('r', 's') => 'p',
        ('r', 'p') => 's',
        ('s', 'r') => 'p',
        ('s', 'p') => 'r',
        ('p', 'r') => 's',
        ('p', 's') => 'r',
        _ => panic!("Invalid input"),
    }
}

fn main() {
    input! {
       n: usize,
       k: usize,
       r: usize,
       s: usize,
       p: usize,
       t: Chars,
    }

    let mut strat: Vec<Option<char>> = vec![None;n];

    for i in 0..n {
        let w = get_winning_play(&t[i]);
        if i < k {
            strat[i] = Some(w);
            continue;
        }
        if strat[i - k].is_none() {
            panic!("Forgot to fill the rank {}", i - k);
        }
        // If we already played the same hand before, we will look forward once to choose the draw or loss hand wisely
        if strat[i - k].unwrap() != w {
            strat[i] = Some(w);
            continue;
        }
        // We are not allowed to play a winning hand - we choose among the two other choices the one that gives us a winning hand next (if any)
        if i + k >= n {
            // in this case the actual play chosen doesn't matter (same score for draw or loss, and no next game impacted by this play)
            strat[i] = Some(get_other_play(&w));
            continue;
        }
        let next_winning_play = get_winning_play(&t[i + k]);
        // We will choose a play which enables us to win the next game
        if next_winning_play == w {
            strat[i] = Some(get_other_play(&w));
            continue;
        }
        strat[i] = Some(get_neither_play(&w, &next_winning_play));
    }

    if DEBUG {
        println!("Strat: {:?}", strat);
    }
    // Now we count the total score
    let mut total_score = 0;
    for i in 0..n {
        if strat[i].is_none() {
            panic!(" Forgot to fill the rank {i}");
        }
        total_score += get_score(&r, &s, &p, &strat[i].unwrap(), &t[i]);
    }

    println!("{}", total_score);
}
