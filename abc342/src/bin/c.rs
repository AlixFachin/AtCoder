use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       s: Chars,
       q: usize,
       queries: [(char, char);q],
    };

    let mut outputs: HashMap<char, char> = HashMap::new();
    let mut dest = s.clone();

    for i in 'a'..='z' {
        outputs.insert(i, i);
    }

    
    for (a, b) in queries {
        
        if outputs.get(&a).unwrap() == &a {
            outputs.insert(a, b);
        }
        
        for i in 'a'..='z' {
            if outputs.get(&i).unwrap() == &a {
                outputs.insert(i, b);
            }
        }
    }

    if DEBUG {
        println!("inputs: {:?}", &outputs);
    }

    for i in 0..n {
        dest[i] = *outputs.get(&dest[i]).unwrap();
    }

    println!("{}", dest.iter().collect::<String>());

}
