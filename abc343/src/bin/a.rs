use proconio::input;

fn main() {
    input! {
       a: usize,
       b: usize,
    };

    if a+ b == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
