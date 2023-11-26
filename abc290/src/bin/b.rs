use proconio::input;
use proconio::marker::Chars;

fn main() {
    
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut result: Vec<char> = vec![' ';n];
    
    let mut num_contest = 1;
    for i in 0..n {
        if s[i] == 'o' && num_contest <= k {
            result[i]= 'o';
            num_contest = num_contest + 1;
        } else {
            result[i]= 'x';
        }
    }

    for i in 0..n {
        print!("{}",result[i]);
    }
    println!("");


}
