use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    };

    let mut v = a.clone();
    v.sort();

    let mut sum_greater: Vec<usize> = vec![0;n];
    let mut sum: usize = 0;
    for i in (0..n).rev() {
        sum_greater[i] = sum;
        sum = sum + v[i];
    }
    let mut b :HashMap<usize, usize> = HashMap::new();
    for i in 0..n-1 {
        if v[i] != v[i+1] {
            b.insert(v[i], sum_greater[i]);
        }
    }
    b.insert(v[n-1],0);

    for i in 0..n {
        print!("{} ", b.get(&a[i]).unwrap());
    }
    println!("");

}
