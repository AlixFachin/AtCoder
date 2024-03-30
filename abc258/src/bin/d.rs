use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       x: usize,
       stages: [(usize, usize); n],
    };

    let mut min_clear_time: usize = std::usize::MAX;
    let mut sum_so_far: usize = 0;
    for i in 0..n {
        let (a,b) = stages[i];
        let clear_i = sum_so_far + a + (x-i)*b;
        min_clear_time = min_clear_time.min(clear_i);
        if DEBUG { println!("min_clear_time: {}, sum so far: {sum_so_far}, a:{a}, b: {b}", min_clear_time);}
        sum_so_far += a + b;
    }
    println!("{}", min_clear_time);
}
