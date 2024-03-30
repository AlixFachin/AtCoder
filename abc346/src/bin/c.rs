use proconio::input;
use std::collections::{hash_set, HashSet};

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       k: usize,
       a: [usize; n],
    };

    let mut hash_set = HashSet::new();
    for i in 0..n {
        hash_set.insert(a[i]);
    }

    let mut sum = k*(k+1)/2;
    for i in hash_set.iter() {
        if *i <= k {
            sum -= *i;
        }
    }

    println!("{}", sum);

}
