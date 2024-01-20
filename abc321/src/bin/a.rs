use proconio::input;
use proconio::marker::Chars;

fn get_num(c: char) -> i32 {
    return c as i32 - '0' as i32;
}

fn main() {
    input! {
       n: Chars,
    };

    for i in 1..n.len() {
        if get_num(n[i]) >= get_num(n[i-1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");

}
