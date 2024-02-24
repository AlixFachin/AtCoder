use std::collections::HashMap;

use proconio::input;

const DEBUG: bool = false;

fn get_prime_list(n: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n + 1 {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn remove_squares(x: usize, prime_list: &Vec<usize>) -> usize {
    let mut x = x;
    for p in prime_list.iter() {
        if p * p > x {
            break;
        }
        while x % (p * p) == 0 {
            x /= p * p;
        }
    }
    x
}

fn main() {
    input! {
       n: usize,
       mut a: [usize; n],
    }

    let prime_list = get_prime_list(200_000);
    for i in 0..n {
        if a[i] == 0 {
            continue;
        }
        a[i] = remove_squares(a[i], &prime_list);
    }

    let mut count_map = HashMap::new();
    let mut count_zero = 0;
    for i in 0..n {
        if a[i] == 0 {
            count_zero += 1;
            continue;
        }
        count_map.entry(a[i]).and_modify(|x|  *x = *x + 1).or_insert(1);
    }

    if DEBUG {
        println!("{:?}", count_map);
    }

    let mut count = 0;
    for (_, v) in count_map.iter() {
        count += (v * (v - 1)) / 2;
    }

    if count_zero > 0 {
        count += count_zero* (n-count_zero) + (count_zero * (count_zero - 1)) / 2;
    }

    println!("{}", count ) ;
}
