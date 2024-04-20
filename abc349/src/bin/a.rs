use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       a: [i32; n-1],
    };

    let sum_a: i32 = a.iter().sum();

    println!("{}", -sum_a);

}
