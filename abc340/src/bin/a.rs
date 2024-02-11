use proconio::input;

fn main() {
    input! {
       a: usize,
       b: usize,
       d: usize,
    };

    let mut cur = a;
    while cur <= b {
        print!("{} ", cur);
        cur += d;
    }
    println!();

}
