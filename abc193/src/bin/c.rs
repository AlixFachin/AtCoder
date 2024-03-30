use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
       n: usize,
    };

    if n <= 2 {
        println!("0");
        return;
    }

    let mut set = HashSet::new();
    let mut i:usize = 2;
    while i <= n {
        if !set.contains(&i) {
            // We will add all the multiples of i
            let mut multiple = i * i;
            while multiple <= n {
                set.insert(multiple);
                multiple = multiple * i;
            }
        }
        i=i+1;
    }
    println!("{}", n - set.len());

}
