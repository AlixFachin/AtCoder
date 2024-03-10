use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
       s: Chars,
    };

    let mut repl_chars: Vec<char> = vec![];
    let mut inside: bool = false;
    for c in s {
        if c == '|' {
            inside = !inside;
        } else if !inside {
            repl_chars.push(c);
        }
    }

    println!("{}", repl_chars.iter().collect::<String>());

}
