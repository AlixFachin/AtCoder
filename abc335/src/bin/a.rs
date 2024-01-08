use proconio::{input, marker::Chars};

fn main() {
    input! {
       s: Chars
    };

    for i in 0..s.len() {
        if i == s.len() - 1 {
            println!("4");
        } else {
            print!("{}", s[i]);
        }
    }

}
