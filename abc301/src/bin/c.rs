use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

const DEBUG:bool = false;

fn is_replaceable(c: char) -> bool {
    return c == 'a' || c == 't' || c == 'c' || c == 'o' || c == 'd' || c == 'e' || c == 'r';
}

fn main() {
    input! {
       s: Chars,
       t: Chars,
    };

    // We basically need to check if there is a permutation of S which is equal to T, modulo the number of jokers

    let mut s_count: HashMap<char, i32> = HashMap::new();
    let mut t_count: HashMap<char, i32> = HashMap::new();
    for c in s {
        let count = s_count.entry(c).or_insert(0);
        *count += 1;
    }
    for c in t {
        let count = t_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut s_joker = s_count.get(&'@').unwrap_or(&0).clone();
    let mut t_joker = t_count.get(&'@').unwrap_or(&0).clone();

    for c in 'a'..='z' {
        
        let s_n = s_count.get(&c).unwrap_or(&0);
        let t_n = t_count.get(&c).unwrap_or(&0);

        if s_n == t_n {
            continue;
        }

        if !is_replaceable(c) && s_n != t_n {
            println!("No");
            return;
        }

        // We have a replaceable case - let's see if we have enough jokers to switch
        if s_n > t_n {
            let diff = s_n - t_n;
            if t_joker >= diff {
                t_joker -= diff;
            } else {
                println!("No");
                return;
            }
        } else {
            let diff = t_n - s_n;
            if s_joker >= diff {
                s_joker -= diff;
            } else {
                println!("No");
                return;
            }
        }
    }

    if s_joker != t_joker {
        println!("No");
        return;
    }
    println!("Yes");

}
