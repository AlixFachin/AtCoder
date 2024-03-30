use proconio::input;
use itertools::Itertools;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
    };

    let s = n.to_string();
    let comb = s.chars().permutations(s.len());
    let mut max_mult = 0;
    for x in comb {        
        if x[0] == '0' {
            continue;
        }
        if DEBUG { println!("Looking at combination: {:?}", x);}
        for l in 1..x.len() {
            if x[l] == '0' {
                continue;
            }
            let a: usize = x[0..l].iter().collect::<String>().parse().unwrap();
            let b: usize = x[l..].iter().collect::<String>().parse().unwrap();
            if DEBUG { println!("l: {l} a: {}, b: {}", a, b);}
            max_mult = max_mult.max(a*b);
        }
    }
    println!("{}", max_mult);

}
