use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       s: Chars,
    };

    let mut i = 0;
    while i < s.len() && s[i] == 'A' {
        i = i + 1;
    }
    while i < s.len() && s[i] == 'B' {
        i = i + 1;
    }
    while i < s.len() && s[i] == 'C' {
        i = i + 1;
    }
    if i != s.len() {
        println!("No");
    } else {
        println!("Yes");
    }


}
