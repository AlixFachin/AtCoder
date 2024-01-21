use proconio::input;
use std::collections::HashMap;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       a: [usize; n],
    };

    // For each element of a, we need to count the number
    // of elements which are strictly greater than it.
    // For better performance, we first sort a and then count the numbers greated than
    // each element

    let mut count_greater = vec![0; n];
    let mut b = a.iter().enumerate().collect::<Vec<(usize,&usize)>>();
    b.sort_by(|a, b| a.1.cmp(b.1));
    let mut current_above: i32 = -1;
    let mut current_value: usize = 0;
    for i in (0..n).rev() {
        if *b[i].1 != current_value {
            current_above += 1;
            current_value = *b[i].1;
            count_greater[b[i].0] = current_above;
        }
        else {
            count_greater[b[i].0] = current_above;
        }
    }
    if DEBUG {
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("count_greater: {:?}", count_greater);
    }

    let mut count_k: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        count_k.entry(count_greater[i] as usize).and_modify(|e| *e += 1).or_insert(1);
    }
    for i in 0..n {
        println!("{}", count_k.get(&i).unwrap_or(&0));
    }

}
