use nalgebra::min;
use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
       s: Chars,
       t: Chars,
    };

    let min_length = s.len().min(t.len());
    for i in 0..min_length {
        if s[i] != t[i] {
            println!("{}", i+1);
            return;
        }
    }
    if s.len() != t.len() {
        println!("{}", min_length+1);
    } else {
        println!("0");
    }


}
