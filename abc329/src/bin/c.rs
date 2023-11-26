use proconio::input;
use proconio::marker::Chars;

use std::collections::{hash_map, HashMap};

const DEBUG : bool = false;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut cur_char: char = s[0];
    let mut cur_len: usize = 1;
    let mut substr_hash: HashMap<(char, usize), usize> = HashMap::new();
    substr_hash.insert( (s[0],1) , 1);
    for i in 1..n {
        if DEBUG { println!("Checking char {} - cur_char is {}", &s[i], &cur_char)}
        if cur_char != s[i] {
            // We change character
            if DEBUG { println!("Changing char (old = {}, new = {}), sub len is {}", &cur_char, &s[i],  &cur_len )}
            substr_hash.insert((cur_char, cur_len  ), 1);
            cur_len = 1;
            cur_char = s[i];
        } else {
            substr_hash.insert((cur_char, cur_len), 1);
            cur_len = cur_len +1;            
        }
    }
    substr_hash.insert((cur_char, cur_len), 1);
    
    println!("{}", substr_hash.len());

}
