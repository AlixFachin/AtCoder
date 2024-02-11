use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
         s: String,
    };

    // We will split the string with the dot
    let ans: Vec<&str> = s.split('.').collect();

    println!("{}", ans[ans.len() -1 ]);

}
