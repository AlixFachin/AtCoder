use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       s: Chars,
    };

    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for c in s {
        *frequencies.entry(c).or_insert(0) += 1;
    }
    let mut max_frequency = 0;
    let mut max_char = 'a';
    for c in 'a'..='z' {
        if frequencies.get(&c).unwrap_or(&0) > &max_frequency {
            max_frequency = *frequencies.get(&c).unwrap_or(&0);
            max_char = c;
        }
    }

    println!("{}", max_char);

}
