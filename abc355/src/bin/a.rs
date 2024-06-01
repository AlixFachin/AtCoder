use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       a: usize,
       b: usize,
    };

    if (a == b) {
        println!("-1");
        return;
    }

    println!("{}", 6 - a - b);
}
