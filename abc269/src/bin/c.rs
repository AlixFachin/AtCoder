use proconio::input;
use itertools::Itertools;

const DEBUG:bool = false;

fn main() {
    input! {
        n: usize,
    };

    let s = format!("{:b}", n);
    let mut bits_indexes: Vec<usize> = vec![];
    for (i,c) in s.chars().enumerate() {
        if c == '1' {
            bits_indexes.push(s.len()-1 -i);
        }
    }
    if DEBUG {
        println!("s: {s}, bits_indexes: {:?}", bits_indexes);
    }
    let mut number_solution: Vec<usize> = vec![];
    for k in 0..=bits_indexes.len() {
        if DEBUG {
            println!("k: {}", k);
        }
        // Let's look at all the combinations of k bits_indexes
        for v in bits_indexes.iter().combinations(k) {
            let mut number = 0;
            for i in v.iter() {
                number += 2usize.pow(**i as u32);
            }
            number_solution.push(number);
            if DEBUG {
                println!("v: {:?}, number={number}", &v);
            }
        }
    }
    number_solution.sort();
    number_solution.dedup();
    for number in number_solution {
        println!("{}", number);
    }

}
