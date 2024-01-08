use proconio::input;

fn main() {
    input! {
       n: usize,
     };
    
     let mut combination_store : Vec<(usize, usize, usize)> = vec![];

    for (i,j,k) in itertools::iproduct!(0..=n, 0..=n, 0..=n) {
        if i + j + k <= n {
            combination_store.push((i,j,k));
        }
    }

    combination_store.sort_by(|a,b| { if a.0 == b.0 && a.1 == b.1 { a.2.cmp(&b.2) } else if a.0 == b.0 { a.1.cmp(&b.1) } else {a.0.cmp(&b.0)}  }   );

    for (i,j,k) in combination_store {
        println!("{} {} {}", i, j, k);
    }


}
