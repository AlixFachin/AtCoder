use std::collections::HashMap;

use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       p: [usize; n],
       q: usize,
       queries: [(usize, usize); q],
    };

    let mut positions: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        positions.insert(p[i], i);
    }

    if DEBUG {
        println!("{:?}", positions);
    }

    for (a,b) in queries {
        let pa = positions.get(&a).unwrap();
        let pb = positions.get(&b).unwrap();
       if pa < pb {
        println!("{a}");
       } else {
        println!("{b}");
       }
    }

}
