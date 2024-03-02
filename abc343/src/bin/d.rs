use std::collections::HashMap;

use proconio::input;
const DEBUG: bool = false;

fn main() {
    input! {
      n: usize,
      t: usize,
      ops: [(usize, usize); t],
    };

    let mut scores: Vec<usize> = vec![0;n];
    let mut count: HashMap<usize, usize> = HashMap::new();

    // There are n players who have a 0 score
    count.insert(0, n);

    for (a,b) in ops {
        if DEBUG {
            println!("a: {}, b: {}", a, b);
            println!("scores: {:?}", scores);
            println!("count: {:?}", count);
        }
        
        // For each operation, we are going to update the scores and the count
        count.entry(scores[a-1]).and_modify(|e| { *e -= 1; });
        if *count.get(&scores[a-1]).unwrap() == 0 {
            count.remove(&scores[a-1]);
        }
        scores[a-1] += b;
        count.entry(scores[a-1]).and_modify(|e| { *e += 1; }).or_insert(1);

        println!("{}", count.len());
    }

}
