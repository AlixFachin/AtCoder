use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;


fn main() {
    input! {
        mut s: Chars,
    }
    let mut stack: Vec<char> = vec![];
    
    for i in 0..s.len() {

        let c = s[i];
        stack.push(c);
        if stack.len() >= 3 {
            let q = stack.len() -1;
            if stack[q] == 'C' && stack[q-1] == 'B' && stack[q-2] == 'A' {
                stack.pop();
                stack.pop();
                stack.pop();
            }
        }
    }

    for c in stack {
        print!("{}",c);
    }

    println!("");

}
