use std::collections::HashMap;

use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
      n : usize,
      a: [i32; n],
    };

    let mut behind: HashMap<usize, usize> = HashMap::new();
    let mut  start_index: usize = 0;
    for i in 0..n {
        let who_is_in_front = a[i];
        if who_is_in_front == -1 {
            start_index = i+1;
        } else {
            behind.insert(who_is_in_front as usize, i+1);
        }
    }
    if DEBUG {
        println!("behind: {:?}", behind);
    }

    let mut current_index = start_index;
    for _i in 0..n{
        print!("{} ", current_index);
        current_index = *behind.get(&current_index).unwrap_or(&0);
    }
    println!();

}
