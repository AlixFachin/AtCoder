use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
       n: usize,
       a: [usize; n],
    };

    // First let's find the frequency of each number
    let mut freq: HashMap<usize, usize>;
    freq = HashMap::new();
    for i in 0..n {
        freq.entry(a[i]).and_modify(|x| *x += 1 ).or_insert(1);
    }

    // Now we need to find how to remove
    let mut to_remove = 0;
    for (k, v) in freq.iter() {
        if v < k {
            to_remove += v;
        } else {
            to_remove += v - k;
        }
    }
    println!("{}", to_remove);

}
