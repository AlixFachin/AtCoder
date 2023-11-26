use proconio::input;
use proconio::marker::{Chars};

fn main() {
    
    input! {
        s: Chars,
    };

    let mut result: Vec<char> = vec![];

    for c in s {
        if c == '0' {
            result.push('0');
        } else if c == '1' {
            result.push('1');
        } else {
            result.pop();
        }
    }

    for c in result {
        print!("{}",c);
    }
    println!("");

}
