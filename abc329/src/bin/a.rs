use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for c in s {
        print!("{} ",c);
    }
    println!("");
}


