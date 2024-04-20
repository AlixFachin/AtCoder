use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
       mut s: Chars,
       t: Chars,
    };

    s.push('x');
    let mut cursor: usize = 0;
    for c in t {
        while cursor < s.len() &&  s[cursor] != c.to_lowercase().nth(0).unwrap()  {
            cursor += 1;
        }
        if cursor == s.len() {
            println!("No");
            return;
        }
        cursor += 1;
    }
    println!("Yes");
}
