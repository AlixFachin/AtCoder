use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

const DEBUG:bool = false;

fn main() {
    input! {
       s: Chars,
    };

    let mut count_chars: HashMap<char, usize> = HashMap::new();
    for c in s {
        count_chars.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    if DEBUG {
        println!("{:?}", count_chars);
    }

    let mut count_hash: HashMap<usize, usize> = HashMap::new();
    for c in count_chars.keys() {
        count_hash.entry(*count_chars.get(c).unwrap()).and_modify(|e| *e += 1).or_insert(1);
    }

    if DEBUG {
        println!("{:?}", count_hash);
    }

    for i in count_hash.keys() {
        if *(count_hash.get(i).unwrap()) != 2 as usize {
            println!("No");
            return;
        }
    }
    println!("Yes");

}
