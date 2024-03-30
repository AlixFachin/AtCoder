use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
       s: Chars
    };

    if s[0] != '<' {
        println!("No");
        return;
    }
    if s[s.len()-1] != '>' {
        println!("No");
        return;
    }
    for i in 1..(s.len()-1) {
        if s[i] != '=' {
            println!("No");
            return;
        }
    }
    println!("Yes");

}
