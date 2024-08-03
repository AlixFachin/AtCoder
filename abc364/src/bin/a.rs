use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       s: [String; n]
    };

    if n == 1 {
        println!("Yes");
        return;
    }

    for i in 1..n-1 {
        if s[i-1] == "sweet" && s[i] == "sweet" {
            println!("No");
            return;
        }
    }
    println!("Yes");

}
