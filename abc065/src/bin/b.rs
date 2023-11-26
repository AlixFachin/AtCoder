use std::collections::HashSet;

use proconio::input;
const INITIAL_BUTTON: usize = 1;
const FINAL_BUTTON: usize = 2;
const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }

    let mut finished = false;
    let mut distance = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    let mut cur_button: usize = INITIAL_BUTTON;
    while !finished {        
        if cur_button == FINAL_BUTTON {
            println!("{}", distance);
            return;
        }
        
        let next_button_index = a[cur_button - 1];
        if DEBUG { println!("At {}, distance = {}, next button is: {}", cur_button, distance, next_button_index)}
        if visited.contains(&next_button_index) {
            println!("-1");
            return;
        }
        distance = distance + 1;
        visited.insert(cur_button);
        cur_button = next_button_index;
    }

}
