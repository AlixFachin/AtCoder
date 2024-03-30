use proconio::input;

const DEBUG: bool = false;

fn get_sum_digits(x: usize) -> usize {
    let mut sum = 0;
    let mut x = x;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}


fn main() {
    input! {
      n: usize,
    };

    // Brute force algorithm
    // Checking if every number below n is divisible by its sum digit

    let mut count_good_integers = 0;
    for i in 1..=n {
        if i % get_sum_digits(i) == 0 {
            count_good_integers += 1;
            if DEBUG {
                println!("{} is good", i);
            }
        }
    }

    println!("{}", count_good_integers);
}
