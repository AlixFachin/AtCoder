use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       a: [i64; n],
    };

    let mut num_passengers = 0;
    let mut min_passengers = 0;

    for i in 0..n {
        num_passengers += a[i];
        min_passengers = min_passengers.min(num_passengers);
    }

    println!("{}", num_passengers-min_passengers);


}
