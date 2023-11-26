use std::collections::HashMap;

use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        d: [usize;n],
    }
    
    let mut possible_dates = HashMap::new();

    possible_dates.insert(1, vec![1,11]);
    possible_dates.insert(2, vec![2,22]);
    possible_dates.insert(3, vec![3,33]);
    possible_dates.insert(4, vec![4,44]);
    possible_dates.insert(5, vec![5,55]);
    possible_dates.insert(6, vec![6,66]);
    possible_dates.insert(7, vec![7,77]);
    possible_dates.insert(8, vec![8,88]);
    possible_dates.insert(9, vec![9,99]);
    possible_dates.insert(11, vec![1,11]);
    possible_dates.insert(22, vec![2,22]);
    possible_dates.insert(33, vec![3,33]);
    possible_dates.insert(44, vec![4,44]);
    possible_dates.insert(55, vec![5,55]);
    possible_dates.insert(66, vec![6,66]);
    possible_dates.insert(77, vec![7,77]);
    possible_dates.insert(88, vec![8,88]);
    possible_dates.insert(99, vec![9,99]);
    
    if DEBUG { println!("{:?}",d)}

    let mut nr_days = 0;
    for i in 0..n {
        if possible_dates.contains_key(&(i + 1) ) {
            if DEBUG { println!("Checking month {}", i+1)}
            let total_dates = possible_dates.get(&(i + 1)).unwrap();
            if DEBUG { println!("Possible dates {:?} for total days {}", total_dates, d[i])}
            
            for day in total_dates {
                if *day <= d[i] {
                    nr_days = nr_days + 1;
                }
            }
        }
    }

    println!("{}", nr_days);




}
