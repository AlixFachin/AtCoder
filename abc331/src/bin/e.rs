use std::{collections::{HashMap, HashSet}, cmp::Ordering};

use proconio::input;

fn main() {
    input! {
       n: usize,
       m: usize,
       l: usize,
       a: [usize;n],
       b: [usize;m],
       yuk: [(usize,usize);l],
    };

    // Find the most expensive meal offered
    let mut main_price_index: Vec<(usize,usize)> = vec![(0,0);n];
    let mut sec_price_index: Vec<(usize,usize)> = vec![(0,0);m];
    let mut yuk_hash: HashSet<(usize,usize)> = HashSet::new();
    for i in 0..n {
        main_price_index[i] = (a[i],i);
    }
    for i in 0..m {
        sec_price_index[i] = (b[i],i);
    }
    for i in 0..l {
        yuk_hash.insert(yuk[i]);
    }

    main_price_index.sort_by(|(a1,_i1),(a2,_i2)| a1.cmp(a2));
    sec_price_index.sort_by(|(a1,_i1),(a2,_i2)| a1.cmp(a2));

    // Now that everything is sorted, we have to find the first non-yuk product, that will be the most expensive
    let mut max_price: usize = 0;
    for i in (0..n).rev() {
        let (price_i, index_i) = main_price_index[i];
        for j in (0..m).rev() {
            let (price_j, index_j) = sec_price_index[j];
            if !yuk_hash.contains(&(index_i +1, index_j+1)) {
                max_price = max_price.max(price_i + price_j);
                break; // we break as no need to look further down the j column
            }
        }
    }
    println!("{}", max_price);

}
