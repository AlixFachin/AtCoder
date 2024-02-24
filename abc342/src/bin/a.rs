use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       s: Chars,
    };

    let mut positions: HashMap<char, Vec<usize>> = HashMap::new();

    for i in 0..s.len() {
        if positions.contains_key(&s[i]) {
            positions.get_mut(&s[i]).unwrap().push(i);
        } else {
            positions.insert(s[i], vec![i]);
        }
    }

    for (k, v) in positions.iter() {
        if v.len() == 1 {
            println!("{}", v[0] + 1);
            return;
        }
    }

}
