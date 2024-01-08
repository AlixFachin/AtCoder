use std::collections::VecDeque;

use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       q: usize,
       queries: [(char, String); q],
    };

    let mut dragon_coord: VecDeque<(i32, i32)> = VecDeque::new();
    for i in 0..n {
        dragon_coord.push_back((i as i32 + 1, 0));
    }

    for (t, d) in queries {
        
        if t == '1' {
            // We move the dragon
            if DEBUG {
                println!("Moving dragon {},{}", &t, &d);
                println!("Before move: {:?}", dragon_coord);
            }
            let mut head_coord = dragon_coord.front().unwrap().clone();
            
            match d.as_str() {
                "L" => head_coord.0 -= 1,
                "R" => head_coord.0 += 1,
                "U" => head_coord.1 += 1,
                "D" => head_coord.1 -= 1,
                _ => panic!("Invalid direction"),
            }            
            dragon_coord.push_front(head_coord);
            dragon_coord.pop_back();
            if DEBUG {
                println!("After move: {:?}", dragon_coord);
            }
        } else {
            // We print the dragon_coord
            let index = d.parse::<usize>().unwrap() - 1;
            println!("{} {}", dragon_coord[index].0, dragon_coord[index].1);
        }
    }
}
