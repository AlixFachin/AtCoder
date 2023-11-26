use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        points: [usize; m],
        solves: [String; n],
    }

    // First, computing the current scores of players
    let mut scores:Vec<usize> = vec![0;n.into()];
    let mut distances:Vec<usize> = vec![0;n];

    for p in 0..n {
        scores[p] = solves[p].chars().enumerate().map(|(idx,c)| if c == 'o' { points[idx] } else { 0 } ).reduce(|acc, x| acc + x).unwrap() + p + 1;
    }

    // println!("scores");
    // println!("{:?}", scores);

    for p in 0..n {
        // First, let's find the max of the other players
        let own_score = scores[p];
        let others_max_score = scores.iter().enumerate().filter(|(idx, _)| *idx != p).reduce( |acc,(idx,val)| if acc.1 < val { (idx,val) } else { acc }  ).unwrap().1;

        // println!("Player {} - own score={}, others max score={}",p,own_score, others_max_score);

        if own_score > *others_max_score {
            distances[p] = 0;
        } else {
            let mut to_solve: Vec<usize> = solves[p].chars().enumerate().filter(|(_idx, c)| *c == 'x').map(|(idx, _)| points[idx]).collect();
            
            to_solve.sort_by(|a,b| b.cmp(a));
            let mut current_score: usize = own_score;
            // Now the answer is the index from which we get above the other_max_score
            for (idx, val) in to_solve.iter().enumerate() {
                current_score = current_score + *val;
                if current_score > *others_max_score {
                    distances[p] = idx + 1;
                    break;
                }
            }
        }
    }

    for p in 0..n {
        println!("{}", distances[p]);
    }

}
