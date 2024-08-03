use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       x: usize,
       y: usize,
       mut a: [usize; n],
       mut b: [usize; n],
    };

    // We need to find the minimum of dishes necessry to get to x or y
    a.sort_by(|a,b| b.cmp(a));
    b.sort_by(|a,b| b.cmp(a));

    // Compute totals
    let mut total_a = 0;
    let mut total_b = 0;
    for i in 0..n {
        total_a += a[i];
        total_b += b[i];
        if total_a >x || total_b > y {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", n);


}
